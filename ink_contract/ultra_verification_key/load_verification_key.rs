use ethnum::{U256, uint};

pub struct Point {
    x: U256,
    y: U256,
}

pub struct C {
    c0: U256,
    c1: U256,
}

pub struct G2X {
    x: C,
    y: C,
}

pub struct VerificationKey {
    circuit_size: U256,
    num_inputs: U256,
    work_root: U256,
    domain_inverse: U256,
    q1: Point,
    q2: Point,
    q3: Point,
    q4: Point,
    q_m: Point,
    q_c: Point,
    q_arithmetic: Point,
    q_sort: Point,
    q_elliptic: Point,
    q_aux: Point,
    q_sigma1: Point,
    q_sigma2: Point,
    q_sigma3: Point,
    q_sigma4: Point,
    q_table1: Point,
    q_table2: Point,
    q_table3: Point,
    q_table_type: Point,
    q_id1: Point,
    q_id2: Point,
    q_id3: Point,
    q_id4: Point,
    contains_recursive_proof: U256,
    recursive_proof_public_input_indices: U256,
    g2_x: G2X,
    work_root_inverse: U256,
}

pub fn load_verification_key() -> U256 {
    uint!("0x5bd9df07721ff64c4d2a2dbd1f1969287d0c790df883df7af4114b5956c1e411")
}