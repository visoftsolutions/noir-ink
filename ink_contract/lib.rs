#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub mod memory;
pub mod ultra_verification_key;
pub mod verify;

#[ink::contract]
mod ink_contract {
    use crate::ultra_verification_key::load_verification_key::VerificationKey;

    #[ink(storage)]
    pub struct BaseUltraVerifier {
        verification_key: VerificationKey,

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
