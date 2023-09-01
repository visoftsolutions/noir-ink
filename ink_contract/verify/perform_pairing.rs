use ethnum::U256;

pub struct PerformPairing<'a> {
    pub pairing_rhs_x_loc: &'a mut U256,
    pub pairing_rhs_y_loc: &'a mut U256,
    pub pairing_lhs_x_loc: &'a mut U256,
    pub pairing_lhs_y_loc: &'a mut U256,
    pub g2x_x0_loc: &'a mut U256,
    pub g2x_x1_loc: &'a mut U256,
    pub g2x_y0_loc: &'a mut U256,
    pub g2x_y1_loc: &'a mut U256,
    pub pairing_success_flag: &'a mut U256,
    pub result_flag: &'a mut U256,
}

fn perform_pairing(input: PerformPairing) {

}
