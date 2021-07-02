pub mod bindings;
pub use bindings::*;

pub mod config;
pub use config::{Config, Network};

pub(crate) fn addr(s: &str) -> ethers::types::Address {
    s.parse().unwrap()
}
