use crate::Base58TxSignature;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TxSignResponse {
    jsonrpc: String,
    id: u8,
    result: Base58TxSignature,
}
