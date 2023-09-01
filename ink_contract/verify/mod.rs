use crate::ink_contract::BaseUltraVerifier;

pub mod perform_pairing;
pub mod load_proof;

pub fn verify(ctx: &mut BaseUltraVerifier, _proof: Vec<u8>, _public_inputs: Vec<[u8; 32]>) -> bool {
    todo!()
}
