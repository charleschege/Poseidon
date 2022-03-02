use crate::{Base58Value, PoseidonError, PoseidonResult};

pub fn base58_to_u32_array(value: Base58Value) -> PoseidonResult<[u8; 32]> {
    let decoded_value = match bs58::decode(&value).into_vec() {
        Ok(decoded) => decoded,
        Err(_) => return Err(PoseidonError::InvalidBase58ForPublickKey),
    };

    let converted_value: [u8; 32] = match decoded_value.try_into() {
        Ok(public) => public,
        Err(_) => return Err(PoseidonError::ErrorConvertingToU832),
    };

    Ok(converted_value)
}

pub const SYSTEM_PROGRAM_ID: [u8; 32] = [0; 32];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoseidonValue {
    Number(u8),
    String(String),
    InvalidValue(String),
}
