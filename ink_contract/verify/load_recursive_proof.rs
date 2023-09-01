use core::{ops::Shl, str::FromStr};

use ethnum::U256;

use crate::{ink_contract::BaseUltraVerifier, memory::constant::Q};

pub fn load_recursive_proof(ctx: &mut BaseUltraVerifier,  _public_inputs: &Vec<u8>) {
    if U256::from_le_bytes(ctx.verification_key.contains_recursive_proof) > 0 {
        let index_counter = U256::from_le_bytes(ctx.verification_key.recursive_proof_public_input_indices).shl(U256::from_str("5").unwrap());
        let (chunk, _) = _public_inputs[index_counter.as_usize()..].as_chunks::<32>();

        let mut x0 = U256::from_le_bytes(chunk[0]);
        x0 += U256::from_le_bytes(chunk[1]).shl(U256::from_str("68").unwrap());
        x0 += U256::from_le_bytes(chunk[2]).shl(U256::from_str("136").unwrap());
        x0 += U256::from_le_bytes(chunk[3]).shl(U256::from_str("204").unwrap());

        let mut y0 = U256::from_le_bytes(chunk[4]);
        y0 += U256::from_le_bytes(chunk[5]).shl(U256::from_str("68").unwrap());
        y0 += U256::from_le_bytes(chunk[6]).shl(U256::from_str("136").unwrap());
        y0 += U256::from_le_bytes(chunk[7]).shl(U256::from_str("204").unwrap());

        let mut x1 = U256::from_le_bytes(chunk[8]);
        x1 += U256::from_le_bytes(chunk[9]).shl(U256::from_str("68").unwrap());
        x1 += U256::from_le_bytes(chunk[10]).shl(U256::from_str("136").unwrap());
        x1 += U256::from_le_bytes(chunk[11]).shl(U256::from_str("204").unwrap());

        let mut y1 = U256::from_le_bytes(chunk[12]);
        y1 += U256::from_le_bytes(chunk[13]).shl(U256::from_str("68").unwrap());
        y1 += U256::from_le_bytes(chunk[14]).shl(U256::from_str("136").unwrap());
        y1 += U256::from_le_bytes(chunk[15]).shl(U256::from_str("204").unwrap());

        ctx.recursion_variable.recursive_p1_x = x0.to_le_bytes();
        ctx.recursion_variable.recursive_p1_y = y0.to_le_bytes();
        ctx.recursion_variable.recursive_p2_x = x1.to_le_bytes();
        ctx.recursion_variable.recursive_p2_y = y1.to_le_bytes();

        if !(x0 < Q && x1 < Q && y0 < Q && y1 < Q) {
            panic!("PUBLIC_INPUT_INVALID_BN128_G1_POINT_SELECTOR");
        }
    }
}