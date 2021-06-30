use derive_more::AsRef;
use ethers::types::U256;
use serde::{de, Deserialize, Serialize};

pub mod planet;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, AsRef)]
#[as_ref]
/// Wrapper type around the planet id, created by the MIMC hash of the coords
pub struct PlanetId(pub U256);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A planet's x,y coordinates on the map
pub struct Coords {
    /// X coordinate
    pub x: i64,
    /// Y coordinate
    pub y: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsRef)]
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

// helper for deserializing hex strings not prefixed with 0x as the PlanetId
fn deserialize_planet_id<'de, D>(deserializer: D) -> Result<PlanetId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom).map(PlanetId)
}
