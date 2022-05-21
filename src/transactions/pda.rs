use crate::{
    AccountMeta, Instruction, PoseidonError, PoseidonResult, PublicKey, SystemInstruction,
    MAX_SEED_LEN, PDA_MARKER,
};
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PdaBuilder {
    from_public_key: PublicKey,
    to_public_key: PublicKey,
    base: PublicKey,
    space: u64,
    owner: PublicKey,
    seed: String,
    lamports: u64,
}

impl PdaBuilder {
    pub fn new() -> Self {
        PdaBuilder {
            from_public_key: PublicKey::default(),
            to_public_key: PublicKey::default(),
            base: PublicKey::default(),
            space: u64::default(),
            owner: PublicKey::default(),
            seed: String::default(),
            lamports: u64::default(),
        }
    }

    pub fn add_from(&mut self, public_key: PublicKey) -> &mut Self {
        self.from_public_key = public_key;

        self
    }

    pub fn add_base(&mut self, public_key: PublicKey) -> &mut Self {
        self.base = public_key;

        self
    }

    pub fn add_space(&mut self, space: u64) -> &mut Self {
        self.space = space;

        self
    }

    pub fn add_owner(&mut self, public_key: PublicKey) -> &mut Self {
        self.owner = public_key;

        self
    }

    pub fn add_seed(&mut self, seed: &str) -> &mut Self {
        self.seed = seed.to_owned();

        self
    }

    pub fn add_lamports(&mut self, lamports: u64) -> &mut Self {
        self.lamports = lamports;

        self
    }

    pub fn derive_public_key(&mut self) -> PoseidonResult<&mut Self> {
        use sha2::{Digest, Sha256};

        if self.seed.len() > MAX_SEED_LEN {
            return Err(PoseidonError::MaxSeedLengthExceeded);
        }

        if self.owner.len() >= PDA_MARKER.len() {
            let slice = &self.owner[self.owner.len() - PDA_MARKER.len()..];
            if slice == PDA_MARKER {
                return Err(PoseidonError::IllegalOwner);
            }
        }

        let mut hasher = Sha256::new();
        hasher.update(&self.base);
        hasher.update(&self.seed);
        hasher.update(&self.owner);

        let sha256_pda: [u8; 32] = hasher.finalize().into();

        self.to_public_key = sha256_pda;

        Ok(self)
    }

    pub fn build(self) -> PoseidonResult<Instruction> {
        let system_instruction = SystemInstruction::CreateAccountWithSeed {
            base: self.base,
            seed: self.seed.to_owned(),
            lamports: self.lamports,
            space: self.space,
            owner: self.owner,
        };

        let data = bincode::serialize(&system_instruction)?;

        Ok(Instruction {
            program_id: crate::SYSTEM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(self.from_public_key, true),
                AccountMeta::new(self.to_public_key, false),
                AccountMeta::new_readonly(self.from_public_key, true),
            ],
            data,
        })
    }
}

impl fmt::Debug for PdaBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PdaBuilder")
            .field(
                "from_public_key",
                &bs58::encode(&self.from_public_key).into_string(),
            )
            .field(
                "to_public_key",
                &bs58::encode(&self.to_public_key).into_string(),
            )
            .field("base", &bs58::encode(&self.base).into_string())
            .field("space", &self.space)
            .field("owner", &bs58::encode(&self.owner).into_string())
            .field("seed", &self.seed)
            .field("lamports", &self.lamports)
            .finish()
    }
}
