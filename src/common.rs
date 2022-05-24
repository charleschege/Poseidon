use borsh::{BorshDeserialize, BorshSerialize};
use generic_array::{typenum::U64, GenericArray};
use serde::{Deserialize, Serialize};

pub type PublicKey = [u8; 32];
pub type RecentBlockHash = [u8; 32];
pub type Signature = GenericArray<u8, U64>;
pub type Base58PublicKey = String;
pub type Base58SecretKey = String;
pub type Base58TxSignature = String;
pub type Base58Signature = String;
pub type Base58BlockHash = String;
pub type BorrowedBase58PublicKey<'pn> = &'pn str;
pub type ProgramID = String;
pub type TxPayer = String;
pub type Base58Value<'a> = &'a str;
pub type ProgramLogEntry = String;
pub type PdaPublicKey = [u8; 32];
pub type UnixTimestamp = i64;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Copy,
    Clone,
)]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
    Unspecified,
}

impl Default for Commitment {
    fn default() -> Self {
        Commitment::Finalized
    }
}

impl From<&str> for Commitment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "processed" => Commitment::Processed,
            "confirmed" => Commitment::Confirmed,
            "finalized" => Commitment::Finalized,
            _ => Commitment::Unspecified,
        }
    }
}

impl Into<&str> for Commitment {
    fn into(self) -> &'static str {
        match self {
            Commitment::Processed => "processed",
            Commitment::Confirmed => "confirmed",
            Commitment::Finalized => "finalized",
            Commitment::Unspecified => "unspecified",
        }
    }
}
