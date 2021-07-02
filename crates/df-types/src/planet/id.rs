use crate::utils;
use derive_more::AsRef;
use ethers::types::U256;
use serde::{de, Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, AsRef, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[as_ref]
/// Wrapper type around the planet id, created by the MIMC hash of the coords
pub struct PlanetId(pub U256);
impl From<U256> for PlanetId {
    fn from(src: U256) -> Self {
        PlanetId(src)
    }
}

impl PlanetId {
    /// Checks if the planet has an energy cap bonus (x2 energy cap)
    pub fn has_energy_cap_bonus(&self) -> bool {
        self.has_bonus(PlanetIdIdx::EnergyCap)
    }

    /// Checks if the planet has an energy growth bonus (x2 energy growth)
    pub fn has_energy_growth_bonus(&self) -> bool {
        self.has_bonus(PlanetIdIdx::EnergyGrowth)
    }

    /// Checks if the planet has an range bonus (x2 range)
    pub fn has_range_bonus(&self) -> bool {
        self.has_bonus(PlanetIdIdx::Range)
    }

    /// Checks if the planet has a speed bonus (x2 speed)
    pub fn has_speed_bonus(&self) -> bool {
        self.has_bonus(PlanetIdIdx::Speed)
    }

    /// Checks if the planet has a defense bonus (x2 defense)
    pub fn has_defense_bonus(&self) -> bool {
        self.has_bonus(PlanetIdIdx::Defense)
    }

    fn has_bonus(&self, idx: PlanetIdIdx) -> bool {
        utils::byte(self.0, idx as usize) < 16
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// The indexes for planet id serialization
pub enum PlanetIdIdx {
    LevelStart = 4,
    LevelEnd = 7,
    TypeByte = 8,
    EnergyCap = 9,
    EnergyGrowth = 10,
    Range = 11,
    Speed = 12,
    Defense = 13,
}

// helper for deserializing hex strings not prefixed with 0x as the PlanetId
pub(crate) fn deserialize_planet_id<'de, D>(deserializer: D) -> Result<PlanetId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom).map(PlanetId)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
/// Bonuses related to the planet's id
pub struct Bonus {
    pub energy_cap: bool,
    pub energy_growth: bool,
    pub range: bool,
    pub speed: bool,
    pub defense: bool,
}

impl From<PlanetId> for Bonus {
    /// A planet's bonuses are calculated by reading the corresponding bytes from its id
    fn from(src: PlanetId) -> Self {
        Self {
            energy_growth: src.has_energy_growth_bonus(),
            energy_cap: src.has_energy_cap_bonus(),
            range: src.has_range_bonus(),
            speed: src.has_speed_bonus(),
            defense: src.has_defense_bonus(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planet_id(src: &str) -> PlanetId {
        src.parse::<U256>().unwrap().into()
    }

    #[test]
    fn many_bonuses() {
        let id = planet_id("0000124403d27fdbb103c3500610b861eadab8f5d2bea9aaa96591f356cdfc50");
        assert!(id.has_energy_cap_bonus());
        assert!(id.has_speed_bonus());
    }

    #[test]
    fn single_bonuses() {
        use PlanetIdIdx::{Defense, EnergyCap, EnergyGrowth, Range, Speed};
        for (id, bonus_type) in &[
            (
                "00008b980c37f8f45d06909888a3a0715cbe85a589cf4340f39c736326295e0a",
                EnergyCap,
            ),
            (
                "0000281108c3800c7aac095a8b53ba04b5595f87dde4b29feeca63bf0c639d11",
                EnergyGrowth,
            ),
            (
                "000035790ad588eaf9d33c047eb6031a708321183b748f43648d607aeb6ee02e",
                Range,
            ),
            (
                "0000124403d27fdbb103c3500610b861eadab8f5d2bea9aaa96591f356cdfc50",
                Speed,
            ),
            (
                "0000820f15b1a4ef16a5f4df8a04057136b7b3932953bfecff470237fe419615",
                Defense,
            ),
        ] {
            match bonus_type {
                PlanetIdIdx::EnergyCap => assert!(planet_id(id).has_energy_cap_bonus()),
                PlanetIdIdx::EnergyGrowth => assert!(planet_id(id).has_energy_growth_bonus()),
                PlanetIdIdx::Range => assert!(planet_id(id).has_range_bonus()),
                PlanetIdIdx::Speed => assert!(planet_id(id).has_speed_bonus()),
                PlanetIdIdx::Defense => assert!(planet_id(id).has_defense_bonus()),
                _ => panic!("unexpected bonus type"),
            }
        }
    }
}
