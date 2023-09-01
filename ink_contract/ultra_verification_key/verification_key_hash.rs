use ethnum::uint;

pub fn verification_key_hash() -> [u8; 32] {
    uint!("0x5bd9df07721ff64c4d2a2dbd1f1969287d0c790df883df7af4114b5956c1e411").to_le_bytes()
}
