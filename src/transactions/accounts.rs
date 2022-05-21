use crate::PublicKey;
use borsh::BorshSerialize;
use core::fmt;
use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone, BorshSerialize, Serialize)]
pub struct AccountMeta {
    /// An account's public key.
    pub pubkey: PublicKey,
    /// True if an `Instruction` requires a `Transaction` signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the account data or metadata may be mutated during program execution.
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn new(pubkey: PublicKey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }
    pub fn new_readonly(pubkey: PublicKey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

impl fmt::Debug for AccountMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccountMeta")
            .field("pubkey", &bs58::encode(&self.pubkey).into_string())
            .field("is_signer", &self.is_signer)
            .field("is_writable", &self.is_writable)
            .finish()
    }
}
