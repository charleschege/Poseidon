use generic_array::{typenum::U64, GenericArray};

pub type PublicKey = [u8; 32];
pub type RecentBlockHash = [u8; 32];
pub type Signature = GenericArray<u8, U64>;
pub type Base58PublicKey = String;
pub type Base58SecretKey = String;
pub type Base58TxSignature = String;
pub type BorrowedBase58PublicKey<'pn> = &'pn str;
pub type ProgramID = String;
pub type TxPayer = String;
pub type Base58Value<'a> = &'a str;
pub type ProgramLogEntry = String;
pub type PdaPublicKey = [u8; 32];
