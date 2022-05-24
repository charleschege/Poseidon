use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use serde::{Deserialize, Serialize};

/// A compact encoding of an instruction.
///
/// A `CompiledInstruction` is a component of a multi-instruction [`Message`],
/// which is the core of a Solana transaction. It is created during the
/// construction of `Message`. Most users will not interact with it directly.
///
/// [`Message`]: crate::Message
#[derive(PartialEq, Eq, Clone, BorshSerialize, BorshDeserialize, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstruction {
    /// Index into the transaction keys array indicating
    /// the program account that executes this instruction.
    pub program_id_index: u8,
    /// Ordered indices into the transaction keys array indicating
    /// which accounts to pass to the program.
    #[serde(with = "short_vec")]
    pub accounts: Vec<u8>,
    /// The program input data.
    #[serde(with = "short_vec")]
    pub data: Vec<u8>,
}

impl fmt::Debug for CompiledInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompiledInstruction")
            .field("program_id_index", &self.program_id_index)
            .field("accounts", &self.accounts)
            .field("data", &blake3::hash(&self.data).to_hex())
            .finish()
    }
}
