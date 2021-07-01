use derive_more::AsRef;
use ethers::types::U256;
use serde::{de, Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, AsRef)]
#[as_ref]
/// Wrapper type around the planet id, created by the MIMC hash of the coords
pub struct PlanetId(pub U256);
impl From<U256> for PlanetId {
    fn from(src: U256) -> Self {
        PlanetId(src)
    }
}

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

// helper for deserializing hex strings not prefixed with 0x as the PlanetId
pub(crate) fn deserialize_planet_id<'de, D>(deserializer: D) -> Result<PlanetId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom).map(PlanetId)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Bonuses related to the planet's id
pub struct Bonus {
    pub energy_cap: u64,
    pub energy_growth: u64,
    pub range: u64,
    pub speed: u64,
    pub defense: u64,
}

impl From<PlanetId> for Bonus {
    fn from(_src: PlanetId) -> Self {
        // depending on which bytes are set on the U256 bytes array
        // done in the `hexgen` package
        Self {
            energy_growth: 1,
            energy_cap: 1,
            range: 1,
            speed: 1,
            defense: 1,
        }
    }
}
