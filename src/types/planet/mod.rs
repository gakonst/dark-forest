//! # Dark Forest Planet Variant Types
//!
//! Planet-related types found in the [types contract](https://github.com/darkforest-eth/eth/blob/master/contracts/DarkForestTypes.sol#L9-L11)
use serde::{Deserialize, Serialize};

mod location;
pub use location::*;

mod info;
pub use info::*;

mod default;
pub use default::DEFAULTS;

mod conversion;

mod upgrades;
pub use upgrades::*;

pub mod id;
pub use id::{Bonus, PlanetId};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
/// The planet's type, depending on which the planet has additional functionalities
pub enum PlanetType {
    /// Simple Planet: Use this for conquering things
    Planet,
    /// Asteroid Field: Asteroid fields produce silver
    SilverMine,
    /// Foundries: Foundries are structures left behind by the Seekers which contain advanced
    /// technologies and artifacts. Artifacts can be harvested at foundries, which in turn
    /// can be found in all biomes except Nebula biomes.
    Ruins,
    /// Spacetime Rip: Within Dark Forest, silver and artifacts have to be ferried around on
    /// voyages between planets. However, Spacetime Rips allow you to warp Dark Forest items in
    /// and out of the Dark Forest universe: you can withdraw silver at a Spacetime Rip to increase
    /// your Round 1 score, and NFT artifacts can be deposited and withdrawn from the Dark Forest
    /// contract to your Ethereum account via these portals as well.
    TradingPost,
    /// Quasar: Quasars are rare objects that can be found in all biomes. Quasars
    /// have unusually high energy and silver capacity, but zero energy growth rate.
    SilverBank,
}

impl Default for PlanetType {
    fn default() -> Self {
        PlanetType::Planet
    }
}

impl From<u8> for PlanetType {
    fn from(src: u8) -> Self {
        match src {
            0 => Self::Planet,
            1 => Self::SilverMine,
            2 => Self::Ruins,
            3 => Self::TradingPost,
            4 => Self::SilverBank,
            _ => panic!("unexpected planet type"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
/// Space types, the deeper the higher level the planets you'll find
pub enum SpaceType {
    /// Planets can be up to Level 4 in this depth
    Nebula,
    /// Planets can be up to Level 5 in this depth
    Space,
    /// Deep space, any kind of planet can be found here
    DeepSpace,
    /// Deepest space, any kind of planet can be found here
    DeadSpace,
}

impl Default for SpaceType {
    fn default() -> Self {
        SpaceType::Nebula
    }
}

impl From<u8> for SpaceType {
    fn from(src: u8) -> Self {
        match src {
            0 => Self::Nebula,
            1 => Self::Space,
            2 => Self::DeepSpace,
            3 => Self::DeadSpace,
            _ => panic!("unexpected planet type"),
        }
    }
}
