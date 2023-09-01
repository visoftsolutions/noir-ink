#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ink_contract {
    #[ink(storage)]
    pub struct BaseUltraVerifier {
        value: bool,
    }

    impl BaseUltraVerifier {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
