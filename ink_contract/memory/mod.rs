mod verification_key;

#[derive(Debug)]
#[ink::storage_item]
pub struct Point {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

#[derive(Debug)]
#[ink::storage_item]
pub struct G2X {
    pub x0: [u8; 32],
    pub x1: [u8; 32],
    pub y0: [u8; 32],
    pub y1: [u8; 32],
}
