use ethers::{types::Address, types::U256};

pub fn addr(s: &str) -> Address {
    s.parse().unwrap()
}

/// read bytes in the <start..end> in reverse order from H256/U256 types
pub fn bytes(buf: U256, start: usize, end: usize) -> Vec<u8> {
    (start..end).map(|i| byte(buf, i)).collect::<Vec<u8>>()
}

/// read a byte in reverse order from a U256 types
pub fn byte(buf: U256, i: usize) -> u8 {
    buf.byte(32 - i - 1)
}
