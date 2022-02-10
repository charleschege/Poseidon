use crate::{
    AccountMeta, Instruction, PoseidonError, PoseidonPublicKey, PoseidonResult, SystemInstruction,
    MAX_SEED_LEN, PDA_MARKER,
};
use core::fmt;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct PdaBuilder<'pn> {
    pub(crate) from_public_key: PoseidonPublicKey,
    pub(crate) to_public_key: PoseidonPublicKey,
    pub(crate) base: PoseidonPublicKey,
    pub(crate) seed: &'pn str,
    pub(crate) lamports: u64,
    pub(crate) space: u64,
    pub(crate) owner: PoseidonPublicKey,
    pub(crate) data: Vec<u8>,
}

impl<'pn> PdaBuilder<'pn> {
    pub fn new() -> Self {
        PdaBuilder {
            from_public_key: PoseidonPublicKey::default(),
            to_public_key: PoseidonPublicKey::default(),
            base: PoseidonPublicKey::default(),
            seed: "POSEIDON",
            lamports: u64::default(),
            space: u64::default(),
            owner: PoseidonPublicKey::default(),
            data: Vec::default(),
        }
    }

    pub fn pda_public_key(&self) -> PoseidonPublicKey {
        self.to_public_key
    }

    pub fn from(&mut self, public_key: PoseidonPublicKey) -> &mut Self {
        self.from_public_key = public_key;

        self
    }

    pub fn to(&mut self, public_key: PoseidonPublicKey) -> &mut Self {
        self.to_public_key = public_key;

        self
    }

    pub fn base(&mut self, public_key: PoseidonPublicKey) -> &mut Self {
        self.base = public_key;

        self
    }

    pub fn lamports(&mut self, lamports_value: u64) -> &mut Self {
        self.lamports = lamports_value;

        self
    }

    pub fn seed(&mut self, seed_value: &'pn str) -> &mut Self {
        self.seed = seed_value;

        self
    }

    pub fn space(&mut self, storage_in_bytes: u64) -> &mut Self {
        self.space = storage_in_bytes;

        self
    }

    pub fn owner(&mut self, public_key: PoseidonPublicKey) -> &mut Self {
        self.owner = public_key;

        self
    }

    pub fn data(&mut self, instruction_data: &[u8]) -> &mut Self {
        self.data = instruction_data.to_owned();

        self
    }

    pub fn derive_public_key(&mut self) -> PoseidonResult<&mut Self> {
        if self.seed.len() > MAX_SEED_LEN {
            return Err(PoseidonError::MaxSeedLengthExceeded);
        }

        // /let owner = owner.as_ref();
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

    pub fn build_pda_instruction(&self) -> PoseidonResult<Instruction> {
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

impl<'pn> Default for PdaBuilder<'pn> {
    fn default() -> Self {
        PdaBuilder::new()
    }
}

impl<'pn> fmt::Debug for PdaBuilder<'pn> {
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
            .field("seed", &self.seed)
            .field("lamports", &self.lamports)
            .field("space", &self.space)
            .field("owner", &bs58::encode(&self.owner).into_string())
            .finish()
    }
}
