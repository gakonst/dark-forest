mod bindings;
pub use bindings::*;

mod config;
pub use config::{Config, Network};

pub(crate) fn addr(s: &str) -> ethers::types::Address {
    s.parse().unwrap()
}
