use crate::{Base58PublicKey, Base58SecretKey, PoseidonResult, ProgramID};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PoseidonConfig {
    pub(crate) secret_key: Base58SecretKey,
    pub public_key: Base58PublicKey,
    pub program_id: ProgramID,
}

impl PoseidonConfig {
    pub fn new() -> Self {
        let default_key: [u8; 32] = [0; 32];
        let default_base58key = bs58::encode(&default_key).into_string();
        PoseidonConfig {
            secret_key: default_base58key.clone(),
            public_key: default_base58key.clone(),
            program_id: default_base58key,
        }
    }

    pub async fn load(&mut self, utf8_path: &Utf8Path) -> PoseidonResult<Self> {
        use async_fs::OpenOptions;
        use futures_lite::io::AsyncReadExt;

        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(utf8_path)
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let config: PoseidonConfig = toml::from_str(&contents)?;

        Ok(config)
    }
}
