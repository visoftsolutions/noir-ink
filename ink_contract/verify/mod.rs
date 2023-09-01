use core::{ops::Shl, str::FromStr};

use ethnum::U256;
use ink::env::hash_bytes;
use ink::env::hash::Keccak256;

use crate::ink_contract::BaseUltraVerifier;

pub mod perform_pairing;
pub mod load_proof;
pub mod load_recursive_proof;

pub fn verify(ctx: &mut BaseUltraVerifier, _proof: Vec<u8>, _public_inputs: Vec<u8>) -> bool {
    load_proof::load_proof(ctx, &_proof);
    load_recursive_proof::load_recursive_proof(ctx, &_public_inputs);

    // Generate initial challenge

    let a = U256::from_le_bytes(ctx.verification_key.n).shl(U256::from_str("224").unwrap());
    let b = U256::from_le_bytes(ctx.verification_key.num_inputs).shl(U256::from_str("224").unwrap());
    
    let mut challenge = [0u8; 32];
    hash_bytes::<Keccak256>([&a.to_le_bytes()[0x00..0x04], &b.to_le_bytes()[0x00..0x04]].concat().as_slice(), &mut challenge);



    todo!()
}
