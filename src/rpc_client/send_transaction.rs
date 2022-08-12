use crate::TransactionError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendTxResponse {
    pub jsonrpc: String,
    pub id: u8,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcTxError {
    pub jsonrpc: String,
    pub id: u8,
    pub error: InnerTxError,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InnerTxError {
    pub code: i16,
    pub message: String,
    pub data: ErrorData,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    pub accounts: Option<String>,
    pub err: TransactionError,
    pub logs: Vec<String>,
    pub units_consumed: u64,
}
