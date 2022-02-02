use crate::{PoseidonError, RpcResponseError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWithSeedResponseError {
    jsonrpc: String,
    id: u8,
    error: ResponseError,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    code: i16,
    message: String,
    data: ResponseData,
}

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    accounts: Option<Vec<String>>,
    //FIXME parse `err: ResponseDataError`,
    //FIXME Parse this logs into a user friendly context
    logs: Vec<String>,
}

impl From<CreateWithSeedResponseError> for PoseidonError {
    fn from(response_data: CreateWithSeedResponseError) -> Self {
        let accounts = if let Some(accounts) = response_data.error.data.accounts {
            accounts
        } else {
            Vec::default()
        };

        PoseidonError::ParsedRpcResponseError {
            jsonrpc: response_data.jsonrpc,
            id: response_data.id,
            json_error_code: response_data.error.code,
            message: response_data.error.message,
            error: RpcResponseError::CreateAccountWithSeedError,
            accounts,
            logs: response_data.error.data.logs,
        }
    }
}
