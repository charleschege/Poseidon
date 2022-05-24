use crate::{
    Base58BlockHash, Base58PublicKey, MessageHeader, PoseidonError, PoseidonResult, RpcClient,
    Transaction, TransactionError, UnixTimestamp,
};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransaction {
    pub jsonrpc: String,
    pub id: u8,
    pub result: Option<RpcResult>,
}

impl GetTransaction {
    pub async fn process(transaction: &str) -> PoseidonResult<GetTransaction> {
        use json::JsonValue;

        let body: json::JsonValue = json::object! {
            jsonrpc: "2.0",
            id: 1u8,
            method: "getTransaction",
            params: json::array![JsonValue::String(transaction.to_owned()), JsonValue::String("base58".to_owned())]
        };

        Ok(GetTransaction::request(body).await?)
    }

    pub fn transaction(&self) -> PoseidonResult<Transaction> {
        match &self.result {
            Some(rpc_result) => {
                let encoded = &rpc_result.transaction.0;
                let decoded = bs58::decode(encoded).into_vec()?;
                let data = bincode::deserialize::<Transaction>(&decoded)?;

                Ok(data)
            }
            None => Err(PoseidonError::TransactionNotFoundInCluster),
        }
    }

    async fn request(body: json::JsonValue) -> PoseidonResult<GetTransaction> {
        let mut rpc = RpcClient::new();
        rpc.add_body(body);
        let response = rpc.send().await?;
        let deser_response: GetTransaction = serde_json::from_str(response.as_str()?)?;

        Ok(deser_response)
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResult {
    pub block_time: UnixTimestamp,
    pub meta: RpcMeta,
    pub transaction: (String, String),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMeta {
    pub err: Option<TransactionError>,
    pub fee: u32,
    pub inner_instructions: Vec<RpcInnerInstructions>,
    pub log_messages: Vec<String>,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub pre_token_balances: Vec<TokenBalances>,
    pub post_token_balances: Vec<TokenBalances>,
    pub rewards: Vec<Reward>,
    pub status: Result<(), TransactionError>,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RpcInnerInstructions {
    pub index: u8,
    pub instructions: Vec<RpcCompiledInstruction>,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalances {
    pub account_index: u8,
    pub mint: Base58PublicKey,
    pub owner: Base58PublicKey,
    pub ui_token_amount: TokenAmount,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: f64,
    pub ui_amount_string: String,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: RewardType,
    pub commission: u8,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    Fee,
    Rent,
    Staking,
    Voting,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]

pub struct RpcCompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: String,
}

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize, BorshSerialize, BorshDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct RpcMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<Base58PublicKey>,
    pub recent_blockhash: Base58BlockHash,
    pub instructions: Vec<RpcCompiledInstruction>,
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
