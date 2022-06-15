use crate::{
    request, request_with_result, Commitment, PoseidonError, PoseidonResult, RpcResponse,
    RpcResponseWithResult,
};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestBlockhash {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

impl GetLatestBlockhash {
    pub async fn process(
        commitment: Commitment,
    ) -> PoseidonResult<RpcResponseWithResult<GetLatestBlockhash>> {
        let commitment: &str = commitment.into();
        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getLatestBlockhash",
            params: json::array![json::object!{
                commitment: commitment,
            }]
        };

        Ok(request_with_result::<GetLatestBlockhash>(body).await?)
    }

    pub fn get_hash(response: RpcResponseWithResult<GetLatestBlockhash>) -> GetLatestBlockhash {
        response.result.value
    }

    pub async fn as_bytes(commitment: Commitment) -> PoseidonResult<[u8; 32]> {
        let response = GetLatestBlockhash::process(commitment).await?;
        let blockhash = GetLatestBlockhash::get_hash(response).blockhash;

        let decoded = bs58::decode(&blockhash).into_vec()?;
        match decoded.try_into() {
            Ok(blockhash) => Ok(blockhash),
            Err(_) => Err(PoseidonError::ErrorConvertingToU832),
        }
    }

    pub fn to_bytes(&self) -> PoseidonResult<[u8; 32]> {
        let decoded = bs58::decode(&self.blockhash).into_vec()?;
        match decoded.try_into() {
            Ok(blockhash) => Ok(blockhash),
            Err(_) => Err(PoseidonError::ErrorConvertingToU832),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFees {
    pub blockhash: String,
    pub fee_calculator: FeeCalculator,
    pub last_valid_block_height: u64,
    pub last_valid_slot: u64,
}

impl GetFees {
    pub async fn process() -> PoseidonResult<RpcResponseWithResult<GetFees>> {
        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getFees",
        };

        Ok(request_with_result::<GetFees>(body).await?)
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeCalculator {
    pub lamports_per_signature: u64,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMinimumBalanceForRentExemption;

impl GetMinimumBalanceForRentExemption {
    pub async fn process<T>() -> PoseidonResult<RpcResponse<u64>> {
        let size = core::mem::size_of::<T>() as u64;

        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getMinimumBalanceForRentExemption",
            params: json::array![
                size
            ]
        };

        Ok(request::<u64>(body).await?)
    }
    pub async fn process_precalculated(size: usize) -> PoseidonResult<RpcResponse<u64>> {
        let size = size as u64;

        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getMinimumBalanceForRentExemption",
            params: json::array![
                size
            ]
        };

        Ok(request::<u64>(body).await?)
    }
}
