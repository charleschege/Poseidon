use crate::{Base58Value, PoseidonError, PoseidonResult};

pub fn base58_to_u32_array(value: Base58Value) -> PoseidonResult<[u8; 32]> {
    let decoded_value = match bs58::decode(&value).into_vec() {
        Ok(decoded) => decoded,
        Err(_) => return Err(PoseidonError::InvalidBase58ForPublicKey),
    };

    let converted_value: [u8; 32] = match decoded_value.try_into() {
        Ok(public) => public,
        Err(_) => return Err(PoseidonError::ErrorConvertingToU832),
    };

    Ok(converted_value)
}

pub fn base58_to_binary(value: Base58Value) -> PoseidonResult<Vec<u8>> {
    match bs58::decode(&value).into_vec() {
        Ok(decoded) => Ok(decoded),
        Err(_) => Err(PoseidonError::InvalidBase58ToVec),
    }
}

pub const SYSTEM_PROGRAM_ID: [u8; 32] = [0; 32];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoseidonValue {
    Number(u8),
    String(String),
    InvalidValue(String),
}
