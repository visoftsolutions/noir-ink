#[derive(Debug)]
#[ink::storage_item]
pub struct RecursionVariable {
    pub recursive_p1_x: [u8; 32],
    pub recursive_p1_y: [u8; 32],
    pub recursive_p2_x: [u8; 32],
    pub recursive_p2_y: [u8; 32],
}
