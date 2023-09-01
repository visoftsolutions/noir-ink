use ethnum::U256;

use crate::ink_contract::BaseUltraVerifier;

pub fn load_recursive_proof(ctx: &mut BaseUltraVerifier) {
    if U256::from_le_bytes(ctx.verification_key.contains_recursive_proof) > 0 {
        //TODO
    }
}