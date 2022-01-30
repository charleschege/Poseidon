use crate::{Message, PoseidonSignature};
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
