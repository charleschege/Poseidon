use crate::{
    utils, Base58PublicKey, BlockHashData, GenericSeaHashMap, Message, MessageBuilder, PdaBuilder,
    PoseidonError, PoseidonJsonValue, PoseidonResult, RecentBlockHashNodeResponse,
    RecentBlockHashResponse, RpcClient, RpcErrorHTTP, RpcMethod, RpcResponseError, SeaHashMap,
    Transaction, TxSignResponse, DEVNET, MAINNET_BETA, TESTNET,
};
use core::fmt;
use generic_array::GenericArray;
use wasmium_securemem::ProtectedEd25519KeyPair;

pub struct Poseidon {
    ed25519_keypair: ProtectedEd25519KeyPair,
    public_keys: SeaHashMap,
    recent_blockhash: BlockHashData,
    environment: &'static str,
    instruction_data: Vec<u8>,
}

impl Poseidon {
    pub fn new(ed25519_keypair: ProtectedEd25519KeyPair) -> Self {
        Poseidon {
            ed25519_keypair,
            public_keys: SeaHashMap::default(),
            recent_blockhash: BlockHashData::default(),
            environment: DEVNET,
            instruction_data: Vec::default(),
        }
    }

    /// Public key of the `ProtectedEd25519KeyPair`
    pub fn protected_public_key(&self) -> [u8; 32] {
        self.ed25519_keypair.public_key()
    }

    pub fn add_public_key(
        &mut self,
        key: &'static str,
        value: Base58PublicKey,
    ) -> PoseidonResult<&mut Self> {
        let public_key = utils::base58_to_u32_array(&value)?;
        self.public_keys.insert(key, public_key);

        Ok(self)
    }

    pub fn add_public_key_array(
        &mut self,
        key: &'static str,
        public_key_array: [u8; 32],
    ) -> &mut Self {
        self.public_keys.insert(key, public_key_array);

        self
    }

    pub fn add_data(&mut self, instruction_data: &[u8]) -> &mut Self {
        self.instruction_data = instruction_data.to_owned();

        self
    }
    pub fn use_devnet(&mut self) -> &mut Self {
        self.environment = DEVNET;

        self
    }

    pub fn use_testnet(&mut self) -> &mut Self {
        self.environment = TESTNET;

        self
    }

    pub fn use_mainnet_beta(&mut self) -> &mut Self {
        self.environment = MAINNET_BETA;

        self
    }

    pub fn get_recent_blockhash(&mut self) -> PoseidonResult<&mut Self> {
        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "processed")
            .add_method(RpcMethod::GetRecentBlockhash)
            .to_json();

        let client_response = RpcClient::new(self.environment).add_body(body).clone();

        let client_response = client_response.send_sync()?;

        let rpc_node_response = client_response.as_str()?;

        let deser_prepare = &mut serde_json::Deserializer::from_str(rpc_node_response);

        let deser_response: Result<RecentBlockHashNodeResponse, _> =
            serde_path_to_error::deserialize(deser_prepare);
        match deser_response {
            Ok(response) => {
                let response: RecentBlockHashResponse = response.into();
                let blockhash = utils::base58_to_u32_array(&response.blockhash)?;
                self.recent_blockhash.add_blockhash(blockhash).owned();

                Ok(self)
            }
            Err(err) => Err(PoseidonError::SerdeJsonDeser(format!("{:?}", err))),
        }
    }

    pub fn get_recent_blockhash_value(&mut self) -> PoseidonResult<RecentBlockHashResponse> {
        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "processed")
            .add_method(RpcMethod::GetRecentBlockhash)
            .to_json();

        let client_response = RpcClient::new(self.environment).add_body(body).clone();

        let client_response = client_response.send_sync()?;

        let rpc_node_response = client_response.as_str()?;

        let deser_prepare = &mut serde_json::Deserializer::from_str(rpc_node_response);

        let deser_response: Result<RecentBlockHashNodeResponse, _> =
            serde_path_to_error::deserialize(deser_prepare);
        match deser_response {
            Ok(response) => {
                let response: RecentBlockHashResponse = response.into();
                let blockhash = utils::base58_to_u32_array(&response.blockhash)?;
                self.recent_blockhash.add_blockhash(blockhash).owned();

                Ok(response)
            }
            Err(err) => Err(PoseidonError::SerdeJsonDeser(format!("{:?}", err))),
        }
    }

    pub fn create_account_with_seed(
        &self,
        pda_builder: PdaBuilder,
    ) -> PoseidonResult<TxSignResponse> {
        let mut message_builder = MessageBuilder::new();
        message_builder
            .add_instruction(pda_builder.build_pda_instruction()?)
            .build();

        let mut message = Message::new();
        message
            .add_recent_blockhash(self.recent_blockhash.blockhash)
            .build(message_builder)?;

        let encoded_message = bincode::serialize(&message)?;

        let signature = self.ed25519_keypair.try_sign(&encoded_message)?;

        let transaction = Transaction {
            signatures: vec![GenericArray::clone_from_slice(&signature.to_bytes())],
            message,
        };

        let serialized_tx = bincode::serialize(&transaction)?;
        let base58_encoded_transaction = bs58::encode(&serialized_tx).into_string();

        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "finalized")
            .add_method(RpcMethod::SendTransaction)
            .add_encoded_data(&base58_encoded_transaction)
            .to_json();

        let client_response = RpcClient::new(self.environment).add_body(body).clone();

        let client_response = client_response.send_sync()?;

        let rpc_node_response = client_response.as_str()?;

        let parsed_response: Result<TxSignResponse, serde_json::Error> =
            serde_json::from_str(rpc_node_response);

        match parsed_response {
            Ok(parsed_response) => Ok(parsed_response),
            Err(_) => {
                let parsed_response: RpcErrorHTTP = serde_json::from_str(rpc_node_response)?;

                let mut rpc_response_error = RpcResponseError::new();
                rpc_response_error.transform(parsed_response);
                Err(rpc_response_error.into())
            }
        }
    }

    pub fn send_transaction(
        &self,
        message_builder: MessageBuilder,
    ) -> PoseidonResult<TxSignResponse> {
        let mut message = Message::new();
        message
            .add_recent_blockhash(self.recent_blockhash.blockhash)
            .build(message_builder)?;

        let encoded_message = bincode::serialize(&message)?;

        let signature = self.ed25519_keypair.try_sign(&encoded_message)?;

        let transaction = Transaction {
            signatures: vec![GenericArray::clone_from_slice(&signature.to_bytes())],
            message,
        };

        dbg!(&transaction);

        let serialized_tx = bincode::serialize(&transaction)?;
        let base58_encoded_transaction = bs58::encode(&serialized_tx).into_string();

        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "finalized")
            .add_method(RpcMethod::SendTransaction)
            .add_encoded_data(&base58_encoded_transaction)
            .to_json();

        let client_response = RpcClient::new(self.environment).add_body(body).clone();

        let client_response = client_response.send_sync()?;

        let rpc_node_response = client_response.as_str()?;

        dbg!(&rpc_node_response);

        let parsed_response: Result<TxSignResponse, serde_json::Error> =
            serde_json::from_str(rpc_node_response);

        match parsed_response {
            Ok(parsed_response) => Ok(parsed_response),
            Err(_) => {
                let parsed_response: RpcErrorHTTP = serde_json::from_str(rpc_node_response)?;

                let mut rpc_response_error = RpcResponseError::new();
                rpc_response_error.transform(parsed_response);
                Err(rpc_response_error.into())
            }
        }
    }
}

impl fmt::Debug for Poseidon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Poseidon")
            .field("ed25519_keypair", &self.ed25519_keypair)
            .field("public_keys", &format_args!("{:?}", &self.public_keys))
            .field("recent_blockhash", &self.recent_blockhash)
            .field("environment", &self.environment)
            .field(
                "instruction_data",
                &blake3::hash(&self.instruction_data).to_hex(),
            )
            .finish()
    }
}

impl fmt::Display for Poseidon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base58_keys: GenericSeaHashMap<&str, String> = GenericSeaHashMap::default();

        self.public_keys.iter().for_each(|(key, value)| {
            base58_keys.insert(key, bs58::encode(value).into_string());
        });

        f.debug_struct("Poseidon")
            .field("ed25519_keypair", &self.ed25519_keypair)
            .field("public_keys", &base58_keys)
            .field("recent_blockhash", &self.recent_blockhash)
            .field("environment", &self.environment)
            .field(
                "instruction_data",
                &blake3::hash(&self.instruction_data).to_hex(),
            )
            .finish()
    }
}
