//! Planet-related information structures, parsed from the DF smart contracts
use derive_more::AsRef;
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

use super::{PlanetType, SpaceType};

#[derive(Default, Clone, Debug, Serialize, Deserialize, AsRef)]
#[as_ref]
/// Wrapper type for the planet's level (0-9)
pub struct PlanetLevel(U256);
impl From<U256> for PlanetLevel {
    fn from(src: U256) -> PlanetLevel {
        PlanetLevel(src)
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, AsRef)]
/// Unified struct for accessing the planet's information
pub struct PlanetInfo {
    #[as_ref]
    /// The planet
    pub planet: Planet,
    #[as_ref]
    /// Extra info about the planet
    pub info: PlanetExtendedInfo,
    #[as_ref]
    /// Information about the planet's coordinates if it's been revealed
    pub coords: RevealedCoords,
}

// https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestTypes.sol
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
// TODO: Add docs
pub struct Planet {
    pub owner: Address,
    pub range: U256,
    pub speed: U256,
    pub defense: U256,
    pub population: U256,
    pub population_cap: U256,
    pub population_growth: U256,
    pub silver_cap: U256,
    pub silver_growth: U256,
    pub silver: U256,
    pub planet_level: PlanetLevel,
    pub planet_type: PlanetType,
    pub is_home: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
// TODO: Add docs
pub struct PlanetExtendedInfo {
    pub initialized: bool,
    pub created_at: U256,
    pub last_updated: U256,
    pub perlin: U256,
    pub space_type: SpaceType,
    pub upgrade_state_0: U256,
    pub upgrade_state_1: U256,
    pub upgrade_state_2: U256,
    pub hat_level: U256,
    pub has_tried_finding_artifact: bool,
    pub prospected_block_number: U256,
    pub destroyed: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
// TODO: Add docs
pub struct RevealedCoords {
    pub location_id: U256,
    pub x: U256,
    pub y: U256,
    pub revealer: Address,
}
