use crate::{Cluster, PoseidonResult, RpcTxError, SendTxResponse, Transaction};
use borsh::{BorshDeserialize, BorshSerialize};
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RpcClient {
    cluster: Cluster,
    headers: Vec<(String, String)>,
    body: JsonValue,
}

impl RpcClient {
    pub fn new() -> Self {
        RpcClient {
            cluster: Cluster::default(),
            headers: vec![("Content-Type".to_owned(), "application/json".to_owned())],
            body: JsonValue::Null,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.push((key.to_owned(), value.to_owned()));

        self
    }

    pub fn add_body(&mut self, body: JsonValue) -> &mut Self {
        self.body = body;

        self
    }

    pub fn common_methods(&mut self, body: JsonValue) -> &mut Self {
        self.add_body(body);

        self
    }

    pub fn prepare_transaction(&mut self, transaction: &Transaction) -> PoseidonResult<&mut Self> {
        let body = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "sendTransaction",
            params: json::array![
                transaction.to_base58()?
            ]
        };

        self.body = body;

        Ok(self)
    }

    pub fn send(&self) -> smol::Task<PoseidonResult<minreq::Response>> {
        let cluster_url = self.cluster.url();
        let body = self.body.clone().to_string();
        let headers = self.headers.clone();

        smol::spawn(async move {
            let mut request = minreq::post(cluster_url).with_body(body);

            for header in headers {
                request = request.with_header(&header.0, &header.1);
            }

            Ok(request.send()?)
        })
    }
}

#[derive(Debug, Clone)]
pub enum TxSendOutcome {
    Success(SendTxResponse),
    Failure(RpcTxError),
}

impl TxSendOutcome {
    pub fn parse_tx(response: minreq::Response) -> PoseidonResult<TxSendOutcome> {
        let first_response = serde_json::from_str::<SendTxResponse>(&response.as_str()?);

        match first_response {
            Ok(value) => Ok(TxSendOutcome::Success(value)),
            Err(first_error) => {
                let err_response = serde_json::from_str::<RpcTxError>(&response.as_str()?);
                match err_response {
                    Ok(value) => Ok(TxSendOutcome::Failure(value)),
                    Err(_) => Err(first_error.into()),
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResponseWithResult<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: RpcResult<T>,
}

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename = "camelCase")]
pub struct RpcResult<T> {
    pub context: Context,
    pub value: T,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Clone,
    Deserialize,
    Serialize,
    BorshSerialize,
    BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
}

pub(crate) async fn request<T: serde::de::DeserializeOwned>(
    body: json::JsonValue,
) -> PoseidonResult<RpcResponse<T>> {
    let mut rpc = RpcClient::new();
    rpc.common_methods(body);
    let response = rpc.send().await?;
    let deser_response: RpcResponse<T> = serde_json::from_str(response.as_str()?)?;

    Ok(deser_response)
}

pub(crate) async fn request_with_result<T: serde::de::DeserializeOwned>(
    body: json::JsonValue,
) -> PoseidonResult<RpcResponseWithResult<T>> {
    let mut rpc = RpcClient::new();
    rpc.common_methods(body);
    let response = rpc.send().await?;
    let deser_response: RpcResponseWithResult<T> = serde_json::from_str(response.as_str()?)?;

    Ok(deser_response)
}
