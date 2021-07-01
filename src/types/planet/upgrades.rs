//! # Planet Upgrades
//!
//! Upgrades cost Silver, and allow you to boost the stats of your planet. You need to move the
//! required silver to a planet to be able to spend it on upgrades.
use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpgradeBranch {
    /// Defense upgrades make your planets less vulnerable to attack    Defense,
    Defense,
    /// Range upgrades make your voyages go further and decay less
    Range,
    ///  Speed upgrades make your voyages go much faster
    Speed,
}

impl From<UpgradeBranch> for U256 {
    fn from(src: UpgradeBranch) -> U256 {
        U256::from(src as u8)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Upgrade {
    pub population_cap: u64,
    pub population_growth: u64,
    pub range: u64,
    pub speed: u64,
    pub defense: u64,
}

impl Default for Upgrade {
    fn default() -> Self {
        Upgrade {
            population_cap: 100,
            population_growth: 100,
            range: 100,
            speed: 100,
            defense: 100,
        }
    }
}

pub type UpgradeLevels = [Upgrade; 4];

pub type RawUpgrade = [[(U256, U256, U256, U256, U256); 4]; 3];
