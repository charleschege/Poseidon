use crate::{request, Commitment, PoseidonError, PoseidonResult, RpcResponse};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestBlockhash {
    blockhash: String,
    last_valid_block_height: u64,
}

impl GetLatestBlockhash {
    pub async fn process(
        commitment: Commitment,
    ) -> PoseidonResult<RpcResponse<GetLatestBlockhash>> {
        let commitment: &str = commitment.into();
        let body: json::JsonValue = json::object! {
            jsonrpc:"2.0",
            id:1,
            method: "getLatestBlockhash",
            params: json::array![json::object!{
                commitment: commitment,
            }]
        };

        Ok(request::<GetLatestBlockhash>(body).await?)
    }

    pub fn get_hash(response: RpcResponse<GetLatestBlockhash>) -> GetLatestBlockhash {
        response.result.value
    }

    pub async fn as_bytes(commitment: Commitment) -> PoseidonResult<[u8; 32]> {
        let response = GetLatestBlockhash::process(commitment).await?;
        let blockhash = GetLatestBlockhash::get_hash(response).blockhash;

        dbg!(&blockhash);
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
    blockhash: String,
    fee_calculator: FeeCalculator,
    last_valid_block_height: u64,
    last_valid_slot: u64,
}

impl GetFees {
    pub async fn process() -> PoseidonResult<RpcResponse<GetFees>> {
        let body: json::JsonValue = json::object! {
            jsonrpc:"2.0",
            id:1,
            method: "getFees",
        };

        Ok(request::<GetFees>(body).await?)
    }
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeCalculator {
    lamports_per_signature: u64,
}
