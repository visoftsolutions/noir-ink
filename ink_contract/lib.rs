#![feature(slice_as_chunks)]
#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod memory;
pub mod ultra_verification_key;
pub mod verify;

#[ink::contract]
mod ink_contract {
    use memory::{challenges::Challenges, local_variables::LocalVariables, misc::Misc, recursion_variable::RecursionVariable, sub_identity::SubIdentity, success_flag::SuccessFlag};

    use crate::{ultra_verification_key::load_verification_key::VerificationKey, memory::proof_data::ProofData};

    #[ink(storage)]
    pub struct BaseUltraVerifier {
        pub public_inputs_hash: [u8; 32],
        pub challenges: Challenges,
        pub local_variables: LocalVariables,
        pub misc: Misc,
        pub proof_data: ProofData,
        pub recursion_variable: RecursionVariable,
        pub sub_identity: SubIdentity,
        pub success_flag: SuccessFlag,
        pub verification_key: VerificationKey,
    }

    impl BaseUltraVerifier {
        #[ink(constructor)]
        pub fn new() -> Self {
            todo!()
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            todo!()
        }
    }
}
