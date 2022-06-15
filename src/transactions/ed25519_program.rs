use crate::{Instruction, ED25519_PROGRAM_ID};
use bytemuck::{bytes_of, Pod, Zeroable};

pub const PUBKEY_SERIALIZED_SIZE: usize = 32;
pub const SIGNATURE_SERIALIZED_SIZE: usize = 64;
pub const SIGNATURE_OFFSETS_SERIALIZED_SIZE: usize = 14;
// bytemuck requires structures to be aligned
pub const SIGNATURE_OFFSETS_START: usize = 2;
pub const DATA_START: usize = SIGNATURE_OFFSETS_SERIALIZED_SIZE + SIGNATURE_OFFSETS_START;

#[derive(Debug)]
pub struct Ed25519ProgramCPI {
    public_key: [u8; 32],
    signature: [u8; 64],
}

impl Ed25519ProgramCPI {
    pub fn new(public_key: [u8; 32]) -> Self {
        Ed25519ProgramCPI {
            public_key: public_key,
            signature: [0u8; 64],
        }
    }

    pub fn add_signature(&mut self, signature: [u8; 64]) -> &mut Self {
        self.signature = signature;

        self
    }

    pub fn build(&self, message: &[u8]) -> Instruction {
        let mut instruction_data = Vec::with_capacity(
            DATA_START
                .saturating_add(SIGNATURE_SERIALIZED_SIZE)
                .saturating_add(PUBKEY_SERIALIZED_SIZE)
                .saturating_add(message.len()),
        );

        let num_signatures: u8 = 1;
        let public_key_offset = DATA_START;
        let signature_offset = public_key_offset.saturating_add(PUBKEY_SERIALIZED_SIZE);
        let message_data_offset = signature_offset.saturating_add(SIGNATURE_SERIALIZED_SIZE);

        // add padding byte so that offset structure is aligned
        instruction_data.extend_from_slice(bytes_of(&[num_signatures, 0]));

        let offsets = Ed25519SignatureOffsets {
            signature_offset: signature_offset as u16,
            signature_instruction_index: u16::MAX,
            public_key_offset: public_key_offset as u16,
            public_key_instruction_index: u16::MAX,
            message_data_offset: message_data_offset as u16,
            message_data_size: message.len() as u16,
            message_instruction_index: u16::MAX,
        };

        instruction_data.extend_from_slice(bytes_of(&offsets));

        debug_assert_eq!(instruction_data.len(), public_key_offset);

        instruction_data.extend_from_slice(&self.public_key);

        debug_assert_eq!(instruction_data.len(), signature_offset);

        instruction_data.extend_from_slice(&self.signature);

        debug_assert_eq!(instruction_data.len(), message_data_offset);

        instruction_data.extend_from_slice(message);

        Instruction {
            program_id: ED25519_PROGRAM_ID,
            accounts: vec![],
            data: instruction_data,
        }
    }
}

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, BorshDeserialize, BorshSerialize)]
#[repr(C)]
pub struct Ed25519SignatureOffsets {
    signature_offset: u16,             // offset to ed25519 signature of 64 bytes
    signature_instruction_index: u16,  // instruction index to find signature
    public_key_offset: u16,            // offset to public key of 32 bytes
    public_key_instruction_index: u16, // instruction index to find public key
    message_data_offset: u16,          // offset to start of message data
    message_data_size: u16,            // size of message data
    message_instruction_index: u16,    // index of instruction data to get message data
}
