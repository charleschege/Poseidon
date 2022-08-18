use borsh::{BorshDeserialize, BorshSerialize};
use generic_array::{typenum::U64, GenericArray};
use serde::{Deserialize, Serialize};

/// The byte representation of an Ed25519 Signature. Stored as a `GenericArray`
/// since Rust dosen't yet support `u256` primitive due to limitations in LLVM compiler.
pub type SignatureGenericArray = GenericArray<u8, U64>;

/// Configures the Solana RPC cluster to connect to
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum Cluster {
    /// A locally run Solana test validator
    LocalNet,
    /// Connect to the developer cluster
    DevNet,
    /// Connect to the testnet cluster for staging
    TestNet,
    /// Connect to the production cluster
    MainNetBeta,
}

impl Cluster {
    /// Convert the cluster selected to a URL
    pub fn url(&self) -> &'static str {
        match self {
            Cluster::LocalNet => "https://127.0.0.1:8899",
            Cluster::DevNet => "https://api.devnet.solana.com",
            Cluster::TestNet => "https://api.testnet.solana.com",
            Cluster::MainNetBeta => "https://api.mainnet-beta.solana.com",
        }
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Cluster::DevNet
    }
}

/// The commitment metric aims to give clients a measure of the network confirmation
/// and stake levels on a particular block.
/// It implements `From<&str>` and `Into<&str>`
#[derive(
    Debug,
    Serialize,
    Deserialize,
    BorshDeserialize,
    BorshSerialize,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Copy,
    Clone,
)]
pub enum Commitment {
    /// A block is processed by RPC servers
    Processed,
    /// A block is has been confirmed
    Confirmed,
    /// A block has been finalized
    Finalized,
    /// The commitment level provided is invalid
    InvalidCommitment,
}

impl Default for Commitment {
    fn default() -> Self {
        Commitment::Finalized
    }
}

impl From<&str> for Commitment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "processed" => Commitment::Processed,
            "confirmed" => Commitment::Confirmed,
            "finalized" => Commitment::Finalized,
            _ => Commitment::InvalidCommitment,
        }
    }
}

impl Into<&str> for Commitment {
    fn into(self) -> &'static str {
        match self {
            Commitment::Processed => "processed",
            Commitment::Confirmed => "confirmed",
            Commitment::Finalized => "finalized",
            Commitment::InvalidCommitment => "invalid_commitment",
        }
    }
}
