#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod clusters;
pub use clusters::*;
mod common;
pub use common::*;
mod transactions;
pub use transactions::*;
mod errors;
pub use errors::*;
mod utilities;
pub use utilities::*;
mod rpc_client;
pub use rpc_client::*;

#[cfg(test)]
mod sanity_checks {
    use crate::*;

    #[test]
    fn request_airdrop() {
        smol::block_on(async {
            let airdrop =
                RequestAirdrop::process("Dvkg2NEkdfqpHrwemwPAuyqoFEZENM7V2DduJRK7QMKr", 2000000000)
                    .await;

            assert!(airdrop.is_ok());
        })
    }
}
