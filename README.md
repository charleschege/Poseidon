### Poseidon - god of the Solana sea 😂

`poseidon-client` is a Minimal Solana Client library that aims to be fast to compile and cache friendly. 

Currently, not all RPC methods are implemented. If you are looking for a feature rich Solana RPC library, take a look at Solana Anchor library or the official Solana SDK and Solana-client.

Current feature support include:

- [x] `getLatestBlockhash`

- [x] `createAccountWithSeed`

- [x] `Message`
- [x] `Instruction`
- [x] `Transaction`
- [x] `getMinimumBalanceForRentExemption`
- [x] `sendTransaction` 

#### Usage

First, generate an Ed25519 Public and Private Keypair or import one. Use the `ed25519_dalek` crate to generate or import the `ed25519_dalek::Keypair`

##### Add dependencies

`File: /Cargo.toml`

```toml
[dependencies]
rand = "0.7"
ed25519-dalek = "*" # add latest version
poseidon-client = "*" # add latest version
```

Or simple use `cargo-edit` crate

```sh
$ cargo add rand --vers 0.7

$ cargo add ed25519-dalek

$ cargo add poseidon
```

##### Generate a new Keypair

```rust
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;

let mut csprng = OsRng{};
let keypair: Keypair = Keypair::generate(&mut csprng);
```

##### Or import the `ed25519_dalek::Keypair` from bytes if you already have one

```rust
use ed25519_dalek::Keypair;

// Example of the 64 bytes of both the secret key (32bytes) and
// public key (32bytes) respectively
let bytes = [0u8; 64];
let keypair: Keypair = Keypair::from_bytes(&bytes)?;
```



#### `CAUTION` : NOTE THAT PROTECTING THE SECRET KEY OF THE KEYPAIR IS BEYOND THE SCOPE OF THIS CRATE. TAKE CARE!!!

##### Create the data structure for serializing and deserializing the storage PDA account of the program

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct PoseidonTestStore {
    username: String,
}
```



##### Get the Minimum rent needed to pay storage costs for 2 years

```rust
use poseidon_client::GetMinimumBalanceForRentExemption;

let two_year_rent =
            GetMinimumBalanceForRentExemption::process::<PoseidonTestStore>().await?;
```

##### Creating a Program Derived Account

```rust
use poseidon_client::{SYSTEM_PROGRAM_ID, PoseidonTestStore, PdaBuilder};

// Decide on the seed to create the PDA account for the program
let seed  = "EXAMPLE_HELLO";

// Instantiate the PDA account builder
let mut pda = PdaBuilder::new();
let pda_public_key = pda
    .add_from(keypair.public.to_bytes())
    .add_base(keypair.public.to_bytes())
    .add_lamports(two_year_rent.result)
    .add_seed(seed)
    .add_space(core::mem::size_of::<PoseidonTestStore>() as u64)
    .add_owner(SYSTEM_PROGRAM_ID)
    .derive_public_key()?;

// Call `build()` to create the `SystemInstruction::CreateAccountWithSeed`
let pda_instruction = pda.build()?;
```

##### Building a Message

```rust
use poseidon_client::MessageBuilder;

let mut message_builder = MessageBuilder::new();
message_builder
    .add_instruction(pda_instruction)
    .add_payer(public_key_bytes)
    .build();

let mut message = Message::new();
message.build(message_builder)?;
```

##### Get Latest Blockhash

```rust
use poseidon_client::GetLatestBlockhash;

let blockhash = GetLatestBlockhash::as_bytes(Commitment::Finalized).await?;
```

##### Create a Transaction

```rust
use ed25519_dalek::Signer;
use poseidon_client::Transaction;

// First update the `recent_blockhash` field of the `Message` with 
// a recent blockhash so that transactions will not fail.
// A Solana `recent_blockhash` only lasts for about `2 minutes` in order to
//prevent replay attacks
message.add_recent_blockhash(blockhash);

// Use the keypair to sign a message
let signature = keypair.sign(&message.to_bytes()?);

// Instantiate a new transaction with the `Message`
let mut transaction = Transaction::new(message);
// Add the signature of the message
transaction.add_signature(signature.to_bytes());
```

##### Send a transaction to a Solana RPC Node

```rust
use poseidon_client::{RpcClient, TxSendOutcome};

let mut rpc = RpcClient::new();
let send_tx_response = rpc.prepare_transaction(&transaction)?.send().await?;
let send_tx_outcome = TxSendOutcome::parse_tx(send_tx_response);
```

##### Get a Transaction using it's hash

```rust
use poseidon_client::GetTransaction;

let base58_signature = "44stjcK4f7RC7KNCorh9gzhQagpYoT9Tq775UFtYbn5gepRocHEeXrtG2JmzgTYKCx83pfBhWHiwLa6sC7f8Ruft";
let tx_resp = GetTransaction::process(sign).await?;
```

### LICENSE

This library is licensed under `MIT` or `Apache-2.0` and all contributions are licensed under the same licenses.

### CODE OF CONDUCT

While making contributions, adhere to the Rust Code of Conduct