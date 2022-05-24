use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum Cluster {
    LocalNet,
    DevNet,
    TestNet,
    MainNetBeta,
}

impl Cluster {
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
