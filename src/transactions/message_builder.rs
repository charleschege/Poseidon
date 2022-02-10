use crate::{AccountMeta, Instruction, PoseidonPublicKey};
use core::fmt;

#[derive(Clone)]
pub struct MessageBuilder {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) program_ids: Vec<AccountMeta>,
    pub(crate) signed_keys: Vec<PoseidonPublicKey>,
    pub(crate) unsigned_keys: Vec<PoseidonPublicKey>,
    pub(crate) num_readonly_signed_accounts: u8,
    pub(crate) num_readonly_unsigned_accounts: u8,
    pub(crate) payer: Option<PoseidonPublicKey>,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        MessageBuilder::new()
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            instructions: Vec::default(),
            program_ids: Vec::default(),
            signed_keys: Vec::default(),
            unsigned_keys: Vec::default(),
            num_readonly_signed_accounts: u8::default(),
            num_readonly_unsigned_accounts: u8::default(),
            payer: Option::default(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);

        self
    }

    pub fn add_payer(&mut self, payer: PoseidonPublicKey) -> &mut Self {
        self.payer = Some(payer);

        self
    }

    pub fn build(&mut self) -> &mut Self {
        self.instructions.iter().for_each(|instruction| {
            self.program_ids.push(AccountMeta {
                pubkey: instruction.program_id,
                is_signer: false,
                is_writable: false,
            });
        });

        let mut instruction_account_metas: Vec<_> = self
            .instructions
            .iter()
            .flat_map(|ix| ix.accounts.iter())
            .collect();

        instruction_account_metas.extend(&self.program_ids);

        // Make signers first
        instruction_account_metas.sort_by(|x, y| {
            y.is_signer
                .cmp(&x.is_signer)
                .then(y.is_writable.cmp(&x.is_writable))
        });

        let payer_account_meta;
        if let Some(payer) = self.payer {
            payer_account_meta = AccountMeta {
                pubkey: payer,
                is_signer: true,
                is_writable: true,
            };

            instruction_account_metas.insert(0, &payer_account_meta);
        }

        let mut unique_account_metas: Vec<AccountMeta> = Vec::default();
        for account_meta in instruction_account_metas {
            // Promote to writable if a later AccountMeta requires it
            if let Some(x) = unique_account_metas
                .iter_mut()
                .find(|x| x.pubkey == account_meta.pubkey)
            {
                x.is_writable |= account_meta.is_writable;
                continue;
            }
            unique_account_metas.push(account_meta.clone());
        }

        unique_account_metas.iter().for_each(|account_meta| {
            if account_meta.is_signer {
                self.signed_keys.push(account_meta.pubkey);
                if !account_meta.is_writable {
                    self.num_readonly_signed_accounts += 1;
                }
            } else {
                self.unsigned_keys.push(account_meta.pubkey);
                if !account_meta.is_writable {
                    self.num_readonly_unsigned_accounts += 1;
                }
            }
        });

        self
    }
}

impl fmt::Debug for MessageBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signed_keys = self
            .signed_keys
            .iter()
            .map(|public_key| bs58::encode(&public_key).into_string())
            .collect::<Vec<String>>();

        let unsigned_keys = self
            .unsigned_keys
            .iter()
            .map(|public_key| bs58::encode(&public_key).into_string())
            .collect::<Vec<String>>();

        f.debug_struct("MessageBuilder")
            .field("instructions", &self.instructions)
            .field("program_ids", &self.program_ids)
            .field("signed_keys", &signed_keys)
            .field("unsigned_keys", &unsigned_keys)
            .field(
                "num_readonly_signed_accounts",
                &self.num_readonly_signed_accounts,
            )
            .field(
                "num_readonly_unsigned_accounts",
                &self.num_readonly_unsigned_accounts,
            )
            .field(
                "payer",
                &match self.payer {
                    Some(payer) => Some(bs58::encode(payer).into_string()),
                    None => None,
                },
            )
            .finish()
    }
}
