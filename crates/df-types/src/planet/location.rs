use crate::{constants, utils::PLANET_BOUNDS};

use super::id::{deserialize_planet_id, PlanetId};

use darkforest_mimc::mimc_hash;
use derive_more::AsRef;
use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
/// A planet's x,y coordinates on the map
pub struct Coords {
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
}

impl From<(i64, i64)> for Coords {
    fn from(src: (i64, i64)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}
impl From<&(i64, i64)> for Coords {
    fn from(src: &(i64, i64)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

#[derive(
    Default, Debug, Clone, Serialize, Deserialize, AsRef, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
/// A planet's location
pub struct PlanetLocation {
    #[as_ref]
    /// The planet's (x, y) coordinates
    pub coords: Coords,
    #[serde(deserialize_with = "deserialize_planet_id")]
    #[as_ref]
    // Technically this is a H256, but the DF contracts interpret it as a uint256,
    // so we prefer to deserialize it this way
    /// The planet's id, created out of the MIMC hash of its coordinates
    pub hash: PlanetId,
    /// The perlin noise value corresponding to the planet
    pub perlin: u64,
    /// ?
    pub biomebase: u64,
}

impl AsRef<U256> for PlanetLocation {
    fn as_ref(&self) -> &U256 {
        self.hash.as_ref()
    }
}

#[derive(Error, Debug, Clone)]
#[error("No planet in the provided coordinates {:?}", .0)]
/// Error type thrown when a planet is not found when converting from x,y coords
/// to hash
pub struct NoPlanet(Coords);

impl TryFrom<&Coords> for PlanetId {
    type Error = NoPlanet;

    fn try_from(coords: &Coords) -> Result<PlanetId, Self::Error> {
        let id = mimc_hash(coords.x, coords.y, constants::PLANETHASH_KEY);

        if &id > &PLANET_BOUNDS {
            return Err(NoPlanet(coords.clone()));
        }

        let id =
            U256::from_dec_str(&id.to_string()).expect("mimc to string to u256 must never fail");

        let id = PlanetId::from(id);
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planet_not_found() {
        let coords = (1139, 126); // no planet exists here
        PlanetId::try_from(&Coords::from(&coords)).unwrap_err();
    }

    #[test]
    fn coords_to_id() {
        for (coords, expected) in &[
            (
                (649i64, 1249i64),
                "442735286437567668657148945899086690986065185626921903368514320303014369",
            ),
            (
                (-74844i64, 100337i64),
                "434915954432531836955876712421785328213695251180755936040562518789811947",
            ),
            (
                (-73378, 97830),
                "246992080379606480896718271497433623460424623349325780632003913836103302",
            ),
        ] {
            assert_eq!(
                PlanetId::try_from(&Coords::from(coords)).unwrap(),
                U256::from_dec_str(expected).unwrap().into()
            );
        }
    }
}
