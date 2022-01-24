use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentBlockHashNodeResponse {
    jsonrpc: String,
    result: Result,
    id: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    context: Context,
    value: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    slot: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    blockhash: String,
    fee_calculator: FeeCalculator,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeCalculator {
    lamports_per_signature: u32,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct RecentBlockHashResponse {
    pub jsonrpc: String,
    pub id: u8,
    pub slot: u64,
    pub blockhash: String,
    pub lamports_per_signature: u32,
}

impl From<RecentBlockHashNodeResponse> for RecentBlockHashResponse {
    fn from(response: RecentBlockHashNodeResponse) -> Self {
        Self {
            jsonrpc: response.jsonrpc,
            id: response.id.unwrap(),
            slot: response.result.context.slot,
            blockhash: response.result.value.blockhash,
            lamports_per_signature: response.result.value.fee_calculator.lamports_per_signature,
        }
    }
}
