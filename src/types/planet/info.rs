//! Planet-related information structures, parsed from the DF smart contracts
use derive_more::AsRef;
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

use super::{Bonus, PlanetLocation, PlanetType, SpaceType, DEFAULTS};

#[derive(Default, Clone, Debug, Serialize, Deserialize, AsRef, PartialEq, Eq, PartialOrd, Ord)]
#[as_ref]
/// Wrapper type for the planet's level (0-9)
pub struct PlanetLevel(U256);
impl From<U256> for PlanetLevel {
    fn from(src: U256) -> PlanetLevel {
        PlanetLevel(src)
    }
}
impl From<u64> for PlanetLevel {
    fn from(src: u64) -> PlanetLevel {
        PlanetLevel(src.into())
    }
}
impl From<u8> for PlanetLevel {
    fn from(src: u8) -> PlanetLevel {
        PlanetLevel(src.into())
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

impl Planet {
    /// Given a planet location, calculates the default planet stats for that planet
    /// before it's initialized
    pub fn apply_defaults(&mut self, loc: &PlanetLocation) {
        self.planet_level = PlanetLevel::from(loc);
        self.planet_type = PlanetType::from(loc);

        let defaults = &DEFAULTS[self.planet_level.as_ref().as_usize()];
        self.population_cap = defaults.population_cap;
        self.population_growth = defaults.population_growth;
        self.range = defaults.range;
        self.speed = defaults.speed;
        self.defense = defaults.defense;
        self.silver_cap = defaults.silver_cap;

        if self.planet_type == PlanetType::SilverMine {
            self.silver_growth = defaults.silver_growth;
        }
    }

    /// Provides 2x bonuses to the planet
    pub fn apply_bonuses<T: Into<Bonus>>(&mut self, bonus: T) {
        let bonus = bonus.into();

        if bonus.energy_cap {
            self.population_cap *= 2;
        }

        if bonus.energy_growth {
            self.population_growth *= 2;
        }

        if bonus.range {
            self.range *= 2;
        }

        if bonus.speed {
            self.speed *= 2;
        }

        if bonus.defense {
            self.defense *= 2;
        }
    }
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

// helper types to convert from the ethers-rs types to a type we can work with more
// easily. ideally, ethers should do this for us.

type RawPlanet = (
    Address,
    U256,
    U256,
    U256,
    U256,
    U256,
    U256,
    U256,
    U256,
    U256,
    U256,
    u8,
    bool,
);
impl From<RawPlanet> for Planet {
    fn from(planet: RawPlanet) -> Self {
        Planet {
            owner: planet.0,
            range: planet.1,
            speed: planet.2,
            defense: planet.3,
            population: planet.4,
            population_cap: planet.5,
            population_growth: planet.6,
            silver_cap: planet.7,
            silver_growth: planet.8,
            silver: planet.9,
            planet_level: PlanetLevel(planet.10),
            planet_type: planet.11.into(),
            is_home: planet.12,
        }
    }
}

type RawPlanetExtendedInfo = (
    bool,
    U256,
    U256,
    U256,
    u8,
    U256,
    U256,
    U256,
    U256,
    bool,
    U256,
    bool,
);
impl From<RawPlanetExtendedInfo> for PlanetExtendedInfo {
    fn from(planet: RawPlanetExtendedInfo) -> Self {
        PlanetExtendedInfo {
            initialized: planet.0,
            created_at: planet.1,
            last_updated: planet.2,
            perlin: planet.3,
            space_type: planet.4.into(),
            upgrade_state_0: planet.5,
            upgrade_state_1: planet.6,
            upgrade_state_2: planet.7,
            hat_level: planet.8,
            has_tried_finding_artifact: planet.9,
            prospected_block_number: planet.10,
            destroyed: planet.11,
        }
    }
}

type RawRevealedCoords = (U256, U256, U256, Address);
impl From<RawRevealedCoords> for RevealedCoords {
    fn from(coords: RawRevealedCoords) -> Self {
        RevealedCoords {
            location_id: coords.0,
            x: coords.1,
            y: coords.2,
            revealer: coords.3,
        }
    }
}
