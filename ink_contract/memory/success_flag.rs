#[derive(Debug)]
#[ink::storage_item]
pub struct SuccessFlag {
    pub grand_product_success_flag: [u8; 32],
    pub arithmetic_term_success_flag: [u8; 32],
    pub batch_opening_success_flag: [u8; 32],
    pub opening_commitment_success_flag: [u8; 32],
    pub pairing_preamble_success_flag: [u8; 32],
    pub pairing_success_flag: [u8; 32],
    pub result_flag: [u8; 32],
}
