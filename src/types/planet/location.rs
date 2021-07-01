use super::id::{deserialize_planet_id, PlanetId};

use derive_more::AsRef;
use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A planet's x,y coordinates on the map
pub struct Coords {
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, AsRef)]
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
