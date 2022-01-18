use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;
/*
{
    "jsonrpc": "2.0",
    "result": {
        "context": {
            "slot": 108604506
        },
        "value": {
            "blockhash": "AMKDaoprEUSipYuNM8fgu7azvSXqvHVdwkscUX2yUjHJ",
            "feeCalculator": {
                "lamportsPerSignature": 5000
            }
        }
    },
    "id": 1
}
*/

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
            id: response.id,
            slot: response.result.context.slot,
            blockhash: response.result.value.blockhash,
            lamports_per_signature: response.result.value.fee_calculator.lamports_per_signature,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "camelCase")]
pub struct RecentBlockHashNodeResponse {
    jsonrpc: String,
    result: Result,
    id: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Result {
    context: Context,
    value: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Context {
    slot: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Value {
    blockhash: String,
    fee_calculator: FeeCalculator,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "camelCase")]
pub struct FeeCalculator {
    lamports_per_signature: u32,
}
