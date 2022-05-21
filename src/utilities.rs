use crate::{Base58Value, PoseidonError, PoseidonResult};

pub struct Utilities;

impl Utilities {
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
            Err(_) => return Err(PoseidonError::InvalidBase58ForPublicKey),
        }
    }
}
