use crate::{
    CompiledInstruction, MessageBuilder, PoseidonError, PoseidonPublicKey, PoseidonResult,
    RecentBlockHash,
};
use borsh::BorshSerialize;
use core::fmt;
use itertools::Itertools;
use serde::Serialize;

#[derive(PartialEq, Eq, Clone, BorshSerialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// The message header, identifying signed and read-only `account_keys`
    /// NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    pub header: MessageHeader,

    /// All the account keys used by this transaction
    #[serde(with = "short_vec")]
    pub account_keys: Vec<PoseidonPublicKey>,

    /// The id of a recent ledger entry.
    pub recent_blockhash: RecentBlockHash,

    /// Programs that will be executed in sequence and committed in one atomic transaction if all
    /// succeed.
    #[serde(with = "short_vec")]
    pub instructions: Vec<CompiledInstruction>,
}

impl Default for Message {
    fn default() -> Self {
        Message::new()
    }
}

impl Message {
    pub fn new() -> Self {
        Self {
            header: MessageHeader::default(),
            account_keys: Vec::default(),
            recent_blockhash: RecentBlockHash::default(),
            instructions: Vec::default(),
        }
    }

    pub fn add_recent_blockhash(&mut self, blockhash: RecentBlockHash) -> &mut Self {
        self.recent_blockhash = blockhash;

        self
    }

    pub fn build(&mut self, message_builder: MessageBuilder) -> PoseidonResult<&mut Self> {
        let mut all_keys = message_builder.signed_keys;
        let num_required_signatures = all_keys.len() as u8;
        all_keys.extend(&message_builder.unsigned_keys);
        self.header.num_required_signatures = num_required_signatures;
        self.header.num_readonly_unsigned_accounts = message_builder.num_readonly_unsigned_accounts;
        self.header.num_readonly_signed_accounts = message_builder.num_readonly_signed_accounts;
        self.account_keys = all_keys.into_iter().unique().collect_vec();
        message_builder
            .instructions
            .iter()
            .try_for_each(|instruction| {
                let program_id_index =
                    match self.account_keys.iter().enumerate().find(|(_, account)| {
                        if *account == &instruction.program_id {
                            true
                        } else {
                            false
                        }
                    }) {
                        Some((index, _)) => index as u8,
                        None => return Err(PoseidonError::ProgramIdNotFound),
                    };

                // FIXME make this more efficient. do it in one loop or flat_map
                let account_indexes = message_builder
                    .instructions
                    .iter()
                    .map(|instruction| {
                        instruction
                            .accounts
                            .iter()
                            .map(|account_meta| {
                                match self.account_keys.iter().enumerate().find(
                                    |(_, public_key)| {
                                        if *public_key == &account_meta.pubkey {
                                            true
                                        } else {
                                            false
                                        }
                                    },
                                ) {
                                    Some((index, _)) => Ok(index as u8),
                                    None => Err(PoseidonError::PublicKeyNotFoundInMessageAccounts),
                                }
                            })
                            .collect::<Result<Vec<u8>, PoseidonError>>()
                            .map_err(|_| PoseidonError::AccountIndexNotFoundInMessageAccounts)
                    })
                    .collect::<Result<Vec<Vec<u8>>, PoseidonError>>()
                    .map_err(|_| PoseidonError::AccountIndexNotFoundInMessageAccounts)?;

                let account_indexes: Vec<u8> = account_indexes.into_iter().flatten().collect();

                self.instructions.push(CompiledInstruction {
                    program_id_index: program_id_index,
                    accounts: account_indexes,
                    data: instruction.data.clone(),
                });

                Ok(())
            })?;

        Ok(self)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let account_keys: Vec<String> = self
            .account_keys
            .iter()
            .map(|account_key| bs58::encode(account_key).into_string())
            .collect();
        f.debug_struct("Message")
            .field("header", &self.header)
            .field("account_keys", &account_keys)
            .field(
                "recent_blockhash",
                &bs58::encode(&self.recent_blockhash).into_string(),
            )
            .field("instructions", &self.instructions)
            .finish()
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, BorshSerialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    /// The number of signatures required for this message to be considered valid. The
    /// signatures must match the first `num_required_signatures` of `account_keys`.
    /// NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    pub num_required_signatures: u8,

    /// The last num_readonly_signed_accounts of the signed keys are read-only accounts. Programs
    /// may process multiple transactions that load read-only accounts within a single PoH entry,
    /// but are not permitted to credit or debit lamports or modify account data. Transactions
    /// targeting the same read-write account are evaluated sequentially.
    pub num_readonly_signed_accounts: u8,

    /// The last num_readonly_unsigned_accounts of the unsigned keys are read-only accounts.
    pub num_readonly_unsigned_accounts: u8,
}
