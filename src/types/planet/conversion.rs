//! Business logic for calculating a planet's type, level as well as its space type
//! from its coordinates
use super::{
    Bonus, PlanetId, PlanetInfo, PlanetLevel, PlanetLocation, PlanetType, SpaceType, DEFAULTS,
};
use crate::constants;
use ethers::types::U256;

// TODO: Finish this implementation
impl From<&PlanetLocation> for PlanetInfo {
    /// Creates a "default" PlanetInfo object to load planets "lazily" when they
    /// have not been instantiated on-chain, given a planet's location
    fn from(loc: &PlanetLocation) -> Self {
        let level = PlanetLevel::from(loc).as_ref().as_usize();
        let planet_type = PlanetType::from(loc);
        let space_type = SpaceType::from(loc);
        let bonus = Bonus::from(loc.hash);

        let defaults = &DEFAULTS[level];
        let mut planet = Self::default();

        // initialize the planet's stats along with any bonuses that go with it
        planet.planet.population_cap = defaults.population_cap * bonus.energy_cap;
        planet.planet.population_growth = defaults.population_growth * bonus.energy_growth;
        planet.planet.range = defaults.range * bonus.range;
        planet.planet.speed = defaults.speed * bonus.speed;
        planet.planet.defense = defaults.defense * bonus.defense;
        planet.planet.silver_cap = defaults.silver_cap;

        if planet_type == PlanetType::SilverMine {
            planet.planet.silver_growth = defaults.silver_growth;
        }

        todo!("https://github.com/darkforest-eth/client/blob/0505b315362b9e87b3c021cdac6515ae3d5bcf09/src/Backend/GameLogic/GameObjects.ts#L1295");

        planet
    }
}

// https://github.com/darkforest-eth/client/blob/050b3e3545f28f559f89a95d41e6b31f916d043a/src/Backend/GameLogic/GameObjects.ts#L1197
impl From<&PlanetLocation> for PlanetLevel {
    /// The PlanetLevel is calculated as the highest level based on the location
    /// and whether the planet's space depth
    fn from(loc: &PlanetLocation) -> Self {
        let space_type = SpaceType::from(loc);
        let level = loc.hash.level().as_ref().as_u64();

        let mut res = constants::PLANET_LEVEL_MIN;
        for lvl in (constants::PLANET_LEVEL_MIN..constants::PLANET_LEVEL_MAX).rev() {
            if lvl < level {
                res = lvl;
                break;
            }
        }

        // clip Nebulas at 4
        if space_type == SpaceType::Nebula && res > 4 {
            res = 4;
        }

        // clip Space at 5
        if space_type == SpaceType::Space && res > 5 {
            res = 5;
        }

        U256::from(res).into()
    }
}

// https://github.com/darkforest-eth/client/blob/050b3e3545f28f559f89a95d41e6b31f916d043a/src/Backend/GameLogic/GameObjects.ts#L1254
impl From<&PlanetLocation> for PlanetType {
    /// The PlanetType is calculated as [..]
    // TODO: Understand this algorithm
    fn from(loc: &PlanetLocation) -> Self {
        // get the planet level from the hash + perlin
        let level = PlanetLevel::from(loc);
        // get the space type from the perlin
        let space_type = SpaceType::from_perlin(loc.perlin);

        // TODO: What is this calculation about?
        let weights =
            &constants::PLANET_TYPE_WEIGHTS[space_type as usize][level.as_ref().as_usize()];
        let weight_sum = weights.iter().sum::<u64>();

        // calculate the thresholds array
        let mut thresholds = vec![weight_sum - weights[0]];
        for i in 1..weights.len() {
            thresholds.push(thresholds[i - 1] - &weights[i]);
        }
        thresholds = thresholds
            .iter()
            .map(|x| ((x * 256) as f64 / weight_sum as f64).floor() as u64)
            .collect::<Vec<_>>();

        // find the type byte that first exceeds a threshold
        let type_byte = loc.hash.type_byte() as u64;
        for i in 0..thresholds.len() {
            if type_byte > thresholds[i] {
                return PlanetType::from(i as u8);
            }
        }

        panic!("Unknown planet type")
    }
}

// TODO: Add a graph showing how each attribute is derived from the planet id
impl PlanetId {
    /// The Planet's type byte is the 8th byte in the 32 byte buffer
    pub fn type_byte(&self) -> u8 {
        self.0.byte(8)
    }

    /// The Planet's level is calculated from the 4-6th bytes in the 32 byte buffer
    pub fn level(&self) -> PlanetLevel {
        U256::from_little_endian(&[self.0.byte(4), self.0.byte(5), self.0.byte(6)]).into()
    }
}

impl SpaceType {
    // https://github.com/darkforest-eth/client/blob/050b3e3545f28f559f89a95d41e6b31f916d043a/src/Backend/GameLogic/GameObjects.ts#L1224
    /// Calculates the Space Type from the provided level of Perlin noise
    pub fn from_perlin(perlin: u64) -> Self {
        if perlin < constants::PERLIN_THRESHOLD_1 {
            SpaceType::Nebula
        } else if perlin < constants::PERLIN_THRESHOLD_2 {
            SpaceType::Space
        } else if perlin < constants::PERLIN_THRESHOLD_3 {
            SpaceType::DeepSpace
        } else {
            SpaceType::DeadSpace
        }
    }
}

impl From<&PlanetLocation> for SpaceType {
    /// The SpaceType is calculated from the planet's perlin noise value
    fn from(loc: &PlanetLocation) -> Self {
        SpaceType::from_perlin(loc.perlin)
    }
}

// Helper functions short-hand for accessing the above From<T> implementations
impl PlanetLocation {
    /// Gets the planet's level
    pub fn level(&self) -> PlanetLevel {
        self.into()
    }

    /// Gets the planet's type
    pub fn planet_type(&self) -> PlanetType {
        self.into()
    }

    /// Gets the planet's space type
    pub fn space_type(&self) -> SpaceType {
        self.into()
    }

    /// Gets the planet's bonuses
    pub fn bonuses(&self) -> Bonus {
        self.hash.into()
    }
}