use crate::RpcMethod;
use core::{fmt, hash::BuildHasherDefault};
use generic_array::{typenum::U64, GenericArray};
use json::{array, object, object::Object, JsonValue};
use seahash::SeaHasher;
use std::collections::HashMap;
use tai64::Tai64N;

pub type PoseidonPublicKey = [u8; 32];
pub type RecentBlockHash = [u8; 32];
pub type PoseidonSignature = GenericArray<u8, U64>;
pub type Base58PublicKey = String;
pub type Base58SecretKey = String;
pub type ProgramID = String;
pub type TxPayer = String;
pub type SeaHashMap = HashMap<&'static str, [u8; 32], BuildHasherDefault<SeaHasher>>;
pub type GenericSeaHashMap<T, U> = HashMap<T, U, BuildHasherDefault<SeaHasher>>;
pub type Base58Value<'a> = &'a str;

pub const DEVNET: &str = "https://api.devnet.solana.com";
pub const TESTNET: &str = "https://api.testnet.solana.com";
pub const MAINNET_BETA: &str = "https://api.mainnet-beta.solana.com";
pub const MAX_SEED_LEN: usize = 32;
pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHashData {
    timestamp: Tai64N,
    pub blockhash: RecentBlockHash,
}

impl BlockHashData {
    pub fn add_blockhash(&mut self, blockhash: RecentBlockHash) -> &Self {
        self.blockhash = blockhash;
        self.timestamp = Tai64N::now();

        self
    }

    pub fn timestamp(&self) -> Tai64N {
        self.timestamp
    }

    pub fn owned(self) -> Self {
        self
    }
}

impl fmt::Debug for BlockHashData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elapsed = self.timestamp.duration_since(&Tai64N::now());

        f.debug_struct("BlockHashData")
            .field(
                "timestamp",
                &format_args!(
                    "{} secs ago",
                    match elapsed {
                        Ok(_) => "Error - FutureTime".to_string(),
                        Err(duration) => humantime::format_duration(duration).to_string(),
                    }
                ),
            )
            .field("blockhash", &bs58::encode(&self.blockhash).into_string())
            .finish()
    }
}

impl core::default::Default for BlockHashData {
    fn default() -> Self {
        BlockHashData {
            timestamp: Tai64N::now(),
            blockhash: [0_u8; 32],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PoseidonJsonValue<'a> {
    jsonrpc: &'a str,
    id: u8,
    method: RpcMethod,
    encoded_data: String,
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> Default for PoseidonJsonValue<'a> {
    fn default() -> Self {
        PoseidonJsonValue::new()
    }
}

impl<'a> PoseidonJsonValue<'a> {
    pub fn new() -> Self {
        PoseidonJsonValue {
            jsonrpc: "2.0",
            id: 1_u8,
            method: RpcMethod::GetAccountInfo,
            encoded_data: String::default(),
            params: Vec::default(),
        }
    }

    pub fn add_parameter(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.params.push((key, value));

        self
    }

    pub fn add_id(&mut self, id: u8) -> &mut Self {
        self.id = id;

        self
    }

    pub fn add_method(&mut self, rpc_method: RpcMethod) -> &mut Self {
        self.method = rpc_method;

        self
    }

    pub fn adjust_jsonrpc_version(&mut self, rpc_version: &'a str) -> &mut Self {
        self.jsonrpc = rpc_version;

        self
    }

    pub fn add_encoded_data(&mut self, encoded_data: &str) -> &mut Self {
        self.encoded_data = encoded_data.to_owned();

        self
    }

    pub fn to_json(&self) -> JsonValue {
        let mut params_object = Object::new();
        self.params
            .iter()
            .for_each(|param| params_object.insert(param.0, param.1.into()));

        if !self.params.is_empty() && !self.encoded_data.is_empty() {
            object! {
                jsonrpc: self.jsonrpc,
                id: self.id,
                method: self.method.to_camel_case(),
                params: array! [
                    JsonValue::String(self.encoded_data.to_owned()),
                    params_object,
                ]
            }
        } else if !self.encoded_data.is_empty() && self.params.is_empty() {
            object! {
                jsonrpc: self.jsonrpc,
                id: self.id,
                method: self.method.to_camel_case(),
                params: array! [
                    JsonValue::String(self.encoded_data.to_owned()),
                ]
            }
        } else {
            object! {
                jsonrpc: self.jsonrpc,
                id: self.id,
                method: self.method.to_camel_case(),
            }
        }
    }
}
