use crate::{
    request, request_with_result, BorrowedBase58PublicKey, Commitment, PoseidonError,
    PoseidonResult, PublicKey, RpcResponse, RpcResponseWithResult,
};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub const LAMPORT: u64 = 1_000_000_000;

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
pub struct RequestAirdrop {
    public_key: PublicKey,
    lamports: u8,
    commitment: Commitment,
}

impl RequestAirdrop {
    pub fn new(public_key: PublicKey) -> Self {
        RequestAirdrop {
            public_key,
            lamports: 2,
            commitment: Commitment::Finalized,
        }
    }

    pub fn add_lamports(&mut self, lamports: u8) -> &mut Self {
        self.lamports = lamports;

        self
    }

    pub fn change_commitment(&mut self, commitment: Commitment) -> &mut Self {
        self.commitment = commitment;

        self
    }

    pub async fn process(&self) -> PoseidonResult<RpcResponse<String>> {
        let public_key = bs58::encode(&self.public_key).into_string();
        let lamports = self.lamports as u64 * LAMPORT;

        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "requestAirdrop",
            params: json::array![public_key, lamports]
        };

        Ok(request::<String>(body).await?)
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

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfo {
    data: (String, String), // (PublicKey String, encoding)
    executable: bool,
    lamports: u64,
    owner: String, // Base58 formatted PublicKey
    rent_epoch: u64,
}

impl GetAccountInfo {
    pub async fn process<'pn>(
        public_key: BorrowedBase58PublicKey<'pn>,
    ) -> PoseidonResult<RpcResponseWithResult<GetAccountInfo>> {
        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getAccountInfo",
            params: [
                public_key,
                {
                    "encoding": "base58"
                }
            ]
        };

        Ok(request_with_result::<GetAccountInfo>(body).await?)
    }
}
