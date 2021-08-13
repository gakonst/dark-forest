pub mod bindings;
pub use bindings::*;

pub mod config;
pub use config::{Config, Network};

pub fn addr(s: &str) -> ethers::types::Address {
    s.strip_prefix("0x").unwrap_or(s).parse().unwrap()
}
