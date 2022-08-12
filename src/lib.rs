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
            let public_key = [
                192, 17, 104, 49, 241, 236, 54, 229, 158, 101, 123, 229, 105, 118, 82, 193, 98,
                254, 160, 8, 178, 16, 110, 239, 141, 143, 116, 88, 155, 176, 244, 205,
            ];
            let airdrop = RequestAirdrop::new(public_key)
                .add_lamports(2)
                .process()
                .await;

            assert!(airdrop.is_ok());
        })
    }
}
