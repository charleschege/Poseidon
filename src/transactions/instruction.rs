use crate::{AccountMeta, PoseidonPublicKey};
use borsh::BorshSerialize;
use core::fmt;
use itertools::Itertools;
use serde::Serialize;

#[derive(PartialEq, Clone, BorshSerialize, Serialize)]
pub struct Instruction {
    /// Pubkey of the program that executes this instruction.
    pub program_id: PoseidonPublicKey,
    /// Metadata describing accounts that should be passed to the program.
    pub accounts: Vec<AccountMeta>,
    /// Opaque data passed to the program for its own interpretation.
    pub data: Vec<u8>,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::new()
    }
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            program_id: [0_u8; 32],
            accounts: Vec::default(),
            data: Vec::default(),
        }
    }

    pub fn add_program_id(&mut self, program_id: PoseidonPublicKey) -> &mut Self {
        self.program_id = program_id;

        self
    }

    pub fn add_account(&mut self, account_meta: AccountMeta) -> &mut Self {
        self.accounts.push(account_meta);

        self
    }

    pub fn add_data(&mut self, instruction_data: &[u8]) -> &mut Self {
        self.data = instruction_data.to_owned();

        self
    }
    pub fn build(&mut self) -> &mut Self {
        let unique_accounts = self.accounts.clone().into_iter().unique().collect_vec();

        self.accounts = unique_accounts;

        self
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("program_id", &bs58::encode(&self.program_id).into_string())
            .field("accounts", &self.accounts)
            .field("data", &blake3::hash(&self.data).to_hex())
            .finish()
    }
}
