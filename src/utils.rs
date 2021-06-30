use ethers::types::Address;

pub fn addr(s: &str) -> Address {
    s.parse().unwrap()
}
