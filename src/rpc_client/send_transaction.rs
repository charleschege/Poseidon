use crate::TransactionError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendTxResponse {
    jsonrpc: String,
    id: u8,
    result: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcTxError {
    jsonrpc: String,
    id: u8,
    error: InnerTxError,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InnerTxError {
    code: i16,
    message: String,
    data: ErrorData,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorData {
    accounts: Option<String>,
    err: TransactionError,
    logs: Vec<String>,
    units_consumed: u64,
}
