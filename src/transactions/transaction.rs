use crate::{Message, PoseidonResult, Signature};
use core::fmt;
use generic_array::GenericArray;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(with = "short_vec")]
    pub signatures: Vec<Signature>,
    pub message: Message,
}

impl Transaction {
    pub fn new(message: Message) -> Self {
        Transaction {
            signatures: Vec::default(),
            message,
        }
    }

    pub fn add_signature(&mut self, signature: [u8; 64]) -> &mut Self {
        self.signatures
            .push(GenericArray::clone_from_slice(&signature));

        self
    }

    pub fn to_bytes(&self) -> PoseidonResult<Vec<u8>> {
        Ok(bincode::serialize(&self)?)
    }

    pub fn to_base58(&self) -> PoseidonResult<String> {
        let serialized_tx = bincode::serialize(&self)?;

        Ok(bs58::encode(&serialized_tx).into_string())
    }
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
