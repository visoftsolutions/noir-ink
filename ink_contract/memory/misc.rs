#[derive(Debug)]
#[ink::storage_item]
pub struct Misc {
    // pub omega_inverse: [u8; 32], moved to verification key
    pub c_alpha_sqr: [u8; 32],
    pub c_alpha_cube: [u8; 32],
    pub c_alpha_quad: [u8; 32],
    pub c_alpha_base: [u8; 32],
}
