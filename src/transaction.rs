use crate::{Base58PublicKey, PoseidonSignature, RecentBlockHash};
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use serde::Serialize;

#[derive(PartialEq, Eq, Clone, Serialize)]
pub struct Transaction {
    #[serde(with = "short_vec")]
    pub signatures: Vec<PoseidonSignature>,
    pub message: Message,
}

impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let signatures: Vec<String> = self
            .signatures
            .iter()
            .map(|signature| bs58::encode(signature).into_string())
            .collect();
        f.debug_struct("Transaction")
            .field("signatures", &signatures)
            .field("message", &self.message)
            .finish()
    }
}

#[derive(PartialEq, Eq, Clone, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// The message header, identifying signed and read-only `account_keys`
    /// NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    pub header: MessageHeader,

    /// All the account keys used by this transaction
    #[serde(with = "short_vec")]
    pub account_keys: Vec<Base58PublicKey>,

    /// The id of a recent ledger entry.
    pub recent_blockhash: RecentBlockHash,

    /// Programs that will be executed in sequence and committed in one atomic transaction if all
    /// succeed.
    #[serde(with = "short_vec")]
    pub instructions: Vec<CompiledInstruction>,
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

    pub fn add_header(&mut self, header: MessageHeader) -> &mut Self {
        self.header = header;

        self
    }

    pub fn add_account_key(&mut self, public_key: Base58PublicKey) -> &mut Self {
        self.account_keys.push(public_key);

        self
    }

    pub fn add_recent_blockhash(&mut self, blockhash: RecentBlockHash) -> &mut Self {
        self.recent_blockhash = blockhash;

        self
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        use itertools::Itertools;

        let mut program_id_index = 0_usize;
        let mut accounts: Vec<u8> = Vec::default();

        instruction
            .accounts
            .iter()
            .enumerate()
            .for_each(|(index, account_key)| {
                self.account_keys.push(account_key.pubkey.clone());

                if instruction.program_id != account_key.pubkey {
                    accounts.push(index as u8);
                } else {
                    //FIXME Error out if the program ID is found
                    program_id_index = index;
                }
            });

        let unique_accounts = accounts.into_iter().unique().collect_vec();

        let compiled_instruction = CompiledInstruction {
            program_id_index: program_id_index as u8,
            accounts: unique_accounts,
            data: instruction.data,
        };

        self.instructions.push(compiled_instruction);

        self
    }

    pub fn num_required_signatures(&mut self, value: u8) -> &mut Self {
        self.header.num_required_signatures = value;

        self
    }

    pub fn num_readonly_signed_accounts(&mut self, value: u8) -> &mut Self {
        self.header.num_readonly_signed_accounts = value;

        self
    }

    pub fn num_readonly_unsigned_accounts(&mut self, value: u8) -> &mut Self {
        self.header.num_readonly_unsigned_accounts = value;

        self
    }

    pub fn build(&mut self, program_id: Base58PublicKey) -> &mut Self {
        // FIXME add support for add `program_id_index` for multiple `CompiledInstruction`s
        let account_keys_len = self.account_keys.len();
        self.account_keys.insert(account_keys_len, program_id);

        self.instructions.iter_mut().for_each(|instruction| {
            instruction.program_id_index = account_keys_len as u8;
        });

        self
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

#[derive(Default, Debug, PartialEq, Eq, Clone, BorshDeserialize, BorshSerialize, Serialize)]
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

/// A compact encoding of an instruction.
///
/// A `CompiledInstruction` is a component of a multi-instruction [`Message`],
/// which is the core of a Solana transaction. It is created during the
/// construction of `Message`. Most users will not interact with it directly.
///
/// [`Message`]: crate::message::Message
#[derive(Debug, PartialEq, Eq, Clone, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct CompiledInstruction {
    /// Index into the transaction keys array indicating the program account that executes this instruction.
    pub program_id_index: u8,
    /// Ordered indices into the transaction keys array indicating which accounts to pass to the program.

    #[serde(with = "short_vec")]
    pub accounts: Vec<u8>,
    /// The program input data.

    #[serde(with = "short_vec")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, BorshSerialize, BorshDeserialize)]
pub struct AccountMeta {
    /// An account's public key.
    pub pubkey: Base58PublicKey,
    /// True if an `Instruction` requires a `Transaction` signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the account data or metadata may be mutated during program execution.
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn new(pubkey: Base58PublicKey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }
    pub fn new_readonly(pubkey: Base58PublicKey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct Instruction {
    /// Pubkey of the program that executes this instruction.
    pub program_id: Base58PublicKey,
    /// Metadata describing accounts that should be passed to the program.
    pub accounts: Vec<AccountMeta>,
    /// Opaque data passed to the program for its own interpretation.
    pub data: Vec<u8>,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            program_id: Base58PublicKey::default(),
            accounts: Vec::default(),
            data: Vec::default(),
        }
    }

    pub fn add_program_id(&mut self, program_id: Base58PublicKey) -> &mut Self {
        self.program_id = program_id;

        self
    }

    pub fn add_account(&mut self, account_meta: AccountMeta) -> &mut Self {
        self.accounts.push(account_meta);

        self
    }

    pub fn add_data<T: BorshSerialize>(&mut self, data: T) -> &mut Self {
        let serialized_data = data.try_to_vec().unwrap(); //FIXME Handle error

        self.data = serialized_data;

        self
    }
    pub fn build(&mut self) -> &mut Self {
        use itertools::Itertools;

        let unique_accounts = self.accounts.clone().into_iter().unique().collect_vec();

        self.accounts = unique_accounts;

        self
    }
}
