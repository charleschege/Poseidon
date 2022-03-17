use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;

use crate::{PoseidonError, PoseidonPublicKey, PoseidonResult};

#[derive(Debug, Deserialize)]
pub struct RpcAccountData {
    jsonrpc: String,
    result: RpcAccountDataResult,
    id: u8,
}
#[derive(Debug, Deserialize)]
pub struct RpcAccountDataResult {
    context: RpcAccountDataContext,
    value: RpcAccountDataValue,
}
#[derive(Debug, Deserialize)]
pub struct RpcAccountDataContext {
    slot: u64,
}
#[derive(Debug, Deserialize)]
pub struct RpcAccountDataValue {
    data: Vec<String>,
    executable: bool,
    lamports: u64,
    owner: String,
    #[serde(rename = "rentEpoch")]
    rent_epoch: u64,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd)]
pub struct AccountData {
    jsonrpc: String,
    id: u8,
    slot: u64,
    data: Vec<String>,
    owner: String,
    rent_epoch: u64,
    executable: bool,
    lamports: u64,
}

impl AccountData {
    pub fn owner_public_key(&self) -> PoseidonResult<PoseidonPublicKey> {
        crate::utils::base58_to_u32_array(self.owner.as_str())
    }

    pub fn all_binary_data(&self) -> Result<Vec<Vec<u8>>, (usize, PoseidonError)> {
        let mut binary_data_collection: Vec<Vec<u8>> = Vec::default();

        for (index, indexed_data) in self.data.iter().enumerate() {
            match crate::utils::base58_to_binary(indexed_data) {
                Ok(binary_data) => binary_data_collection.push(binary_data),
                Err(error) => return Err((index, error)),
            }
        }

        Ok(binary_data_collection)
    }

    pub fn indexed_binary_data(&self, index: usize) -> PoseidonResult<Vec<u8>> {
        match bs58::decode(&self.data[index]).into_vec() {
            Ok(binary_data) => Ok(binary_data),
            Err(_) => Err(PoseidonError::InvalidBase58ToVec),
        }
    }
}

impl From<RpcAccountData> for AccountData {
    fn from(rpc_data: RpcAccountData) -> Self {
        AccountData {
            jsonrpc: rpc_data.jsonrpc,
            id: rpc_data.id,
            slot: rpc_data.result.context.slot,
            data: rpc_data.result.value.data,
            owner: rpc_data.result.value.owner,
            rent_epoch: rpc_data.result.value.rent_epoch,
            executable: rpc_data.result.value.executable,
            lamports: rpc_data.result.value.lamports,
        }
    }
}
