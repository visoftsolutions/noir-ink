#[derive(Debug)]
#[ink::storage_item]
pub struct LocalVariables {
    pub delta_numerator: [u8; 32],
    pub delta_denominator: [u8; 32],
    pub zeta_pow_n: [u8; 32],
    pub public_input_delta: [u8; 32],
    pub zero_poly: [u8; 32],
    pub l_start: [u8; 32],
    pub l_end: [u8; 32],
    pub r_zero_eval: [u8; 32],

    pub plookup_delta_numerator: [u8; 32],
    pub plookup_delta_denominator: [u8; 32],
    pub plookup_delta: [u8; 32],

    pub accumulator_x: [u8; 32],
    pub accumulator_y: [u8; 32],
    pub accumulator2_x: [u8; 32],
    pub accumulator2_y: [u8; 32],
    pub pairing_lhs_x: [u8; 32],
    pub pairing_lhs_y: [u8; 32],
    pub pairing_rhs_x: [u8; 32],
    pub pairing_rhs_y: [u8; 32],
}
