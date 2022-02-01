//! ### Poseidon  
//!
//! Poseidon is a Solana development framework to make it easy to develop programs for
//! Solana and Wasmium Network.
//!
//! ##### Creating a PDA
//! The PDA account is created and added to the `PdaBuilder` struct by calling
//! `build()` method on `PdaBuilder`
//! ```rust,no_run
//! use poseidon::{PdaBuilder, SYSTEM_PROGRAM_ID, ProtectedEd25519KeyPair};
//! use borsh::{BorshSerialize, BorshDeserialize};
//!
//! // Create a `ProtectedEd25519KeyPair` from `wasmium-securemem` crate.
//! // This will protect the private key of the keypair in memory.
//! // Here  `KEYPAIR_BYTES` is an `Ed25519` keypair in bytes, which is `[u8;64]` type
//! let keypair = ProtectedEd25519KeyPair::from_bytes(&KEYPAIR_BYTES).unwrap();
//! let pda_builder = PdaBuilder::new();
//!    pda_builder
//!        .from(keypair.public_key())
//!        .base(keypair.public_key())
//!        .seed("A_SEED_<_21_BYTES") //A seed phrase maximum length `PDA_MARKER` constant
//!        .lamports(8000000) //Number of SOL to send to new PDA account equal to 2 years rent
//!        .space(std::mem::size_of::<ExampleInstruction>() as u64)
//!        .owner(SYSTEM_PROGRAM_ID); //Which Public Key manages the new PDA account
//!        .build() // Returns an `Instruction`
//!
//!
//! #[derive(BorshSerialize, BorshDeserialize)]
//! pub struct ExampleInstruction {
//!     pub lamports: u64,
//! }
//!
//! ```
//!
//!
//! ### Sending a Transaction
//! Transactions are sent using `Poseidon`
//! ```rust,no_run
//! use poseidon::{PdaBuilder, SYSTEM_PROGRAM_ID, ProtectedEd25519KeyPair};
//! use borsh::{BorshSerialize, BorshDeserialize};
//!
//! let keypair = ProtectedEd25519KeyPair::from_bytes(&KEYPAIR_BYTES).unwrap();
//! let mut poseidon = Poseidon::new(keypair);
//! poseidon
//!     .get_recent_blockhash()?;
//!
//! // Create a `ProtectedEd25519KeyPair` from `wasmium-securemem` crate.
//! // This will protect the private key of the keypair in memory.
//! // Here  `KEYPAIR_BYTES` is an `Ed25519` keypair in bytes, which is `[u8;64]` type
//! let pda_builder = PdaBuilder::new();
//!    pda_builder
//!        .from(poseidon.protected_public_key())
//!        .base(poseidon.protected_public_key())
//!        .seed("A_SEED_<_21_BYTES") //A seed phrase maximum length `PDA_MARKER` constant
//!        .lamports(8000000) //Number of SOL to send to new PDA account equal to 2 years rent
//!        .space(std::mem::size_of::<ExampleInstruction>() as u64)
//!        .owner(SYSTEM_PROGRAM_ID); //Which Public Key manages the new PDA account
//!        .build() // Returns an `Instruction`
//!
//! poseidon.create_account_with_seed(pda_builder)?;
//!
//!
//! #[derive(BorshSerialize, BorshDeserialize)]
//! pub struct ExampleInstruction {
//!     pub lamports: u64,
//! }
//! ```

mod common;
/// Common utilities
pub use common::*;
/// Data structures for sending transactions
mod transactions;
pub use transactions::*;
/// Enum for creation of an RPC call and it's conversion to `camelCase`
mod rpc_methods;
pub use rpc_methods::*;
#[cfg(features = "smol_async_io")]
mod fs_crypto;
/// Structures and methods for fetching secrets and configuration
/// from the filesystem
#[cfg(features = "smol_async_io")]
pub use fs_crypto::*;
/// Error handling module
mod errors;
pub use errors::*;
mod rpc_client;
/// Methods for making GET and POST requests to Solana RPC endpoints
pub use rpc_client::*;
mod poseidon;
/// Handle all operations in one structure
pub use poseidon::*;
mod rpc_responses;
/// Decodes Solana RPC JSON responses
pub use rpc_responses::*;
pub mod utils;
/// Utilities used across various modules
pub use utils::*;
/// Re-export of `wasmium-securemem` crate
pub use wasmium_securemem::*;
mod system_instruction;
/// Module for Native Solana Program System Instruction data structures
pub use system_instruction::*;
