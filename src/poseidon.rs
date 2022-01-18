use crate::{
    utils, Base58PublicKey, BlockHashData, PoseidonJsonValue, PoseidonResult,
    RecentBlockHashNodeResponse, RecentBlockHashResponse, RpcClient, RpcMethod, SeaHashMap, DEVNET,
    MAINNET_BETA, TESTNET,
};
use wasmium_securemem::ProtectedEd25519KeyPair;

#[derive(Debug)]
pub struct Poseidon {
    ed25519_keypair: ProtectedEd25519KeyPair,
    public_keys: SeaHashMap,
    recent_blockhash: BlockHashData,
    deploy_env: &'static str,
}

impl Poseidon {
    pub fn new(ed25519_keypair: ProtectedEd25519KeyPair) -> Self {
        Poseidon {
            ed25519_keypair,
            public_keys: SeaHashMap::default(),
            recent_blockhash: BlockHashData::default(),
            deploy_env: DEVNET,
        }
    }

    pub fn add_keys(
        &mut self,
        key: &'static str,
        value: Base58PublicKey,
    ) -> PoseidonResult<&mut Self> {
        let public_key = utils::base58_to_u32_array(&value)?;
        self.public_keys.insert(key, public_key);

        Ok(self)
    }

    pub fn use_devnet(&mut self) -> &mut Self {
        self.deploy_env = DEVNET;

        self
    }

    pub fn use_testnet(&mut self) -> &mut Self {
        self.deploy_env = TESTNET;

        self
    }

    pub fn use_mainnet_beta(&mut self) -> &mut Self {
        self.deploy_env = MAINNET_BETA;

        self
    }

    pub fn get_recent_blockhash(&mut self) -> PoseidonResult<&mut Self> {
        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "processed")
            .add_method(RpcMethod::GetRecentBlockhash)
            .to_json();

        let response = RpcClient::new(self.deploy_env).add_body(body).send_sync()?;
        let response: RecentBlockHashNodeResponse = serde_json::from_str(response.as_str()?)?;
        let response: RecentBlockHashResponse = response.into();
        let blockhash = utils::base58_to_u32_array(&response.blockhash)?;

        self.recent_blockhash = BlockHashData::default().add_blockhash(blockhash).owned();

        Ok(self)
    }

    pub fn get_recent_blockhash_value(&mut self) -> PoseidonResult<RecentBlockHashResponse> {
        let body = PoseidonJsonValue::new()
            .add_parameter("commitment", "processed")
            .add_method(RpcMethod::GetRecentBlockhash)
            .to_json();

        let client_response = RpcClient::new(self.deploy_env).add_body(body).send_sync()?;
        let deser_response: RecentBlockHashNodeResponse =
            serde_json::from_str(client_response.as_str()?)?;
        let response: RecentBlockHashResponse = deser_response.into();

        Ok(response)
    }
}
