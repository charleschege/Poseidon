use generic_array::{typenum::U64, GenericArray};

pub type Base58PublicKey = [u8; 32];
pub type RecentBlockHash = [u8; 32];
pub type PoseidonSignature = GenericArray<u8, U64>;
