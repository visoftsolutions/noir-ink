#[derive(Debug)]
#[ink::storage_item]
pub struct SubIdentity {
    pub permutation_identity: [u8; 32],
    pub plookup_identity: [u8; 32],
    pub arithmetic_identity: [u8; 32],
    pub sort_identity: [u8; 32],
    pub elliptic_identity: [u8; 32],
    pub aux_identity: [u8; 32],
    pub aux_non_native_field_evaluation: [u8; 32],
    pub aux_limb_accumulator_evaluation: [u8; 32],
    pub aux_ram_consistency_evaluation: [u8; 32],
    pub aux_rom_consistency_evaluation: [u8; 32],
    pub aux_memory_evaluation: [u8; 32],
}
