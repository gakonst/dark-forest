//! # Dark Forest
//!
//! Dark Forest is a decentralized MMO strategy game, built on Ethereum and xDAI
//! with zero-knowledge cryptography. Players explore a massive and procedurally-generated
//! universe, conquering planets and growing a cosmic empire.
//!
//! Learn more at the [blog](https://blog.zkga.me)

mod bindings;

mod utils;

mod config;
pub use config::{Config, Network};

pub mod constants;

pub mod types;
