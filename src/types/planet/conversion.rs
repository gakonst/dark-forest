//! Business logic for calculating a planet's type, level as well as its space type
//! from its coordinates
use super::{
    Bonus, PlanetId, PlanetInfo, PlanetLevel, PlanetLocation, PlanetType, SpaceType, DEFAULTS,
};
use crate::constants;
use ethers::types::{H256, U256};

// TODO: Finish this implementation
impl From<&PlanetLocation> for PlanetInfo {
    /// Creates a "default" PlanetInfo object to load planets "lazily" when they
    /// have not been instantiated on-chain, given a planet's location
    fn from(loc: &PlanetLocation) -> Self {
        let level = PlanetLevel::from(loc);
        let planet_type = PlanetType::from(loc);
        let space_type = SpaceType::from(loc);
        let bonus = Bonus::from(loc.hash);

        let defaults = &DEFAULTS[level.as_ref().as_usize()];
        let mut planet = Self::default();

        // initialize the planet's rarities
        planet.planet.planet_level = level;
        planet.planet.planet_type = planet_type;
        planet.info.space_type = space_type;

        // initialize the stats & bonuses
        planet.planet.population_cap = defaults.population_cap * bonus.energy_cap;
        planet.planet.population_growth = defaults.population_growth * bonus.energy_growth;
        planet.planet.range = defaults.range * bonus.range;
        planet.planet.speed = defaults.speed * bonus.speed;
        planet.planet.defense = defaults.defense * bonus.defense;
        planet.planet.silver_cap = defaults.silver_cap;

        if planet_type == PlanetType::SilverMine {
            planet.planet.silver_growth = defaults.silver_growth;
        }

        // todo!("https://github.com/darkforest-eth/client/blob/0505b315362b9e87b3c021cdac6515ae3d5bcf09/src/Backend/GameLogic/GameObjects.ts#L1295");

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
            if level < constants::PLANET_LEVEL_THRESHOLDS[lvl as usize] {
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
            if type_byte >= thresholds[i] {
                return PlanetType::from(i as u8);
            }
        }

        let err = format!("Unknown planet type {:#?}", loc);
        panic!("{}", err)
    }
}

// TODO: Add a graph showing how each attribute is derived from the planet id
impl PlanetId {
    pub fn as_hash(&self) -> H256 {
        let mut bytes = [0; 32];
        self.0.to_big_endian(&mut bytes);
        H256::from(bytes)
    }

    /// The Planet's type byte is the 8th byte in the 32 byte buffer
    pub fn type_byte(&self) -> u8 {
        self.byte(8)
    }

    /// The Planet's level is calculated from the 4-6th bytes in the 32 byte buffer
    pub fn level(&self) -> PlanetLevel {
        U256::from_big_endian(&self.bytes(4, 7)).into()
    }

    pub fn bytes(&self, start: usize, end: usize) -> Vec<u8> {
        (start..end).map(|i| self.byte(i)).collect::<Vec<u8>>()
    }

    // bytes are read in reverse order
    fn byte(&self, i: usize) -> u8 {
        self.0.byte(32 - i - 1)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::planet::{location::deserialize_planet_id, Coords};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct PlanetTest {
        inputs: Inputs,
        intermediate_values: IntermediateValues,
        outputs: Outputs,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Inputs {
        #[serde(deserialize_with = "deserialize_planet_id")]
        hex: PlanetId,
        perlin: u64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct IntermediateValues {
        weights: [u64; 5],
        weight_sum: u64,
        thresholds: [u64; 5],
        type_byte: u8,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Outputs {
        planet_level: u8,
        planet_type: u8,
    }

    #[test]
    fn planet_level() {
        let hash = "0000290408f351956089c045b04e062f7bbc8ae8cedfffed0e2a1f7f7c028139"
            .parse::<U256>()
            .unwrap();
        let hash = PlanetId::from(hash);
        // manually obtained value by running with the same args in the official
        // client
        assert_eq!(hash.level().as_ref().as_u64(), 586577);

        let perlin = 13;
        let location = PlanetLocation {
            hash,
            perlin,
            ..Default::default()
        };

        let level = PlanetLevel::from(&location);
        assert_eq!(level.as_ref().as_u64(), 2);
    }

    #[test]
    fn planet_stats() {
        // extracted by manually console log'ing and getting certain tests from this function
        // https://github.com/darkforest-eth/client/blob/050b3e3545f28f559f89a95d41e6b31f916d043a/src/Backend/GameLogic/GameObjects.ts#L1254
        let data = std::fs::read_to_string("./test-vectors/many-planets.json").unwrap();
        let planet_tests: Vec<PlanetTest> = serde_json::from_str(&data).unwrap();

        for case in planet_tests.into_iter() {
            let location = PlanetLocation {
                hash: case.inputs.hex,
                perlin: case.inputs.perlin,
                // these don't matter
                coords: Coords::default(),
                biomebase: 0,
            };

            assert_eq!(location.level(), case.outputs.planet_level.into());
            assert_eq!(
                location.planet_type(),
                PlanetType::from(case.outputs.planet_type)
            );
            // v0.6r2 is all Deep Space
            assert_eq!(location.space_type(), SpaceType::DeepSpace);
        }
    }
}
