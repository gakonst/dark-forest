//! Business logic for calculating a planet's type, level as well as its space type
//! from its coordinates
use crate::{
    constants,
    planet::{
        id::{Bonus, PlanetId, PlanetIdIdx},
        info::{Planet, PlanetExtendedInfo, PlanetInfo, PlanetLevel},
        location::PlanetLocation,
        PlanetType, SpaceType,
    },
    utils,
};
use ethers::types::{H256, U256};

impl From<&PlanetLocation> for PlanetInfo {
    /// Creates a "default" PlanetInfo object to load planets "lazily" when they
    /// have not been instantiated on-chain, given a planet's location
    fn from(loc: &PlanetLocation) -> Self {
        PlanetInfo {
            planet: Planet::new(loc),
            info: PlanetExtendedInfo {
                space_type: SpaceType::from(loc),
                ..Default::default()
            },
            coords: Default::default(),
        }
    }
}

// https://github.com/darkforest-eth/client/blob/050b3e3545f28f559f89a95d41e6b31f916d043a/src/Backend/GameLogic/GameObjects.ts#L1197
impl From<&PlanetLocation> for PlanetLevel {
    /// The PlanetLevel is calculated as the highest level based on the location
    /// and whether the planet's space depth
    fn from(loc: &PlanetLocation) -> Self {
        let space_type = SpaceType::from(loc);
        let level = loc.hash.level().as_ref().as_u32();

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
            thresholds.push(thresholds[i - 1] - weights[i]);
        }
        thresholds = thresholds
            .iter()
            .map(|x| ((x * 256) as f64 / weight_sum as f64).floor() as u64)
            .collect::<Vec<_>>();

        // find the type byte that first exceeds a threshold
        let type_byte = loc.hash.type_byte() as u64;
        for (i, threshold) in thresholds.iter().enumerate() {
            if &type_byte >= threshold {
                return PlanetType::from(i as u8);
            }
        }

        let err = format!("Unknown planet type {:#?}", loc);
        panic!("{}", err)
    }
}

impl PlanetId {
    pub fn as_hash(&self) -> H256 {
        let mut bytes = [0; 32];
        self.0.to_big_endian(&mut bytes);
        H256::from(bytes)
    }

    /// The Planet's type byte is the 8th byte in the 32 byte buffer
    pub fn type_byte(&self) -> u8 {
        self.byte(PlanetIdIdx::TypeByte)
    }

    /// The Planet's level is calculated from the 4-6th bytes in the 32 byte buffer
    pub fn level(&self) -> PlanetLevel {
        U256::from_big_endian(&self.bytes(PlanetIdIdx::LevelStart, PlanetIdIdx::LevelEnd)).into()
    }

    /// Returns the (start..end)-th byte in reverse order from the PlanetId
    pub fn bytes(&self, start: PlanetIdIdx, end: PlanetIdIdx) -> Vec<u8> {
        utils::bytes(self.0, start as usize, end as usize)
    }

    /// Returns the i-th byte in reverse order from the PlanetId
    pub fn byte(&self, i: PlanetIdIdx) -> u8 {
        utils::byte(self.0, i as usize)
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
    use crate::{
        planet::{id::deserialize_planet_id, Coords, PlanetId},
        utils::root_path,
    };
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
        let path = root_path("../../test-vectors/many-planets.json");
        dbg!(&path);
        let data = std::fs::read_to_string(path).unwrap();
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
