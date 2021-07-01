//! # Dark Forest Constants
//!
//! Includes constants found throughout the Dark Forest system (v0.6r2)
//!
//! Most are taken from the
//! [DarkForest.toml](https://github.com/darkforest-eth/eth/blob/ea476ffcba67ca4d826bd15c01691dabea233db3/darkforest.toml)

pub const TARGET4_RADIUS: u64 = 1600;
pub const INITIAL_WORLD_RADIUS: u64 = 8000;

////////////////////////////////////
// SNARK keys & Perlin parameters //
////////////////////////////////////

pub const PLANETHASH_KEY: u32 = 430;
pub const SPACETYPE_KEY: u64 = 431;
pub const BIOMEBASE_KEY: u64 = 432;
pub const PERLIN_MIRROR_X: bool = false;
pub const PERLIN_MIRROR_Y: bool = false;
pub const PERLIN_LENGTH_SCALE: u64 = 16384; // must be a power of two no greater than 16384

////////////////////////
// Game configuration //
////////////////////////

pub const MAX_NATURAL_PLANET_LEVEL: u64 = 256;
pub const TIME_FACTOR_HUNDREDTHS: u64 = 100; // speedup/slowdown game
pub const PERLIN_THRESHOLD_1: u64 = 1;
pub const PERLIN_THRESHOLD_2: u64 = 2;
pub const PERLIN_THRESHOLD_3: u64 = 18;
pub const INIT_PERLIN_MIN: u64 = 14;
pub const INIT_PERLIN_MAX: u64 = 15;
pub const BIOME_THRESHOLD_1: u64 = 15;
pub const BIOME_THRESHOLD_2: u64 = 17;
pub const PLANET_RARITY: u32 = 16384;
pub const PHOTOID_ACTIVATION_DELAY: u64 = 14400; // seconds
pub const LOCATION_REVEAL_COOLDOWN: u64 = 86400; // seconds

pub const PLANET_TYPE_WEIGHTS: &[&[[u64; 5]]] = &[
    &[
        [1, 0, 0, 0, 0],
        [13, 2, 0, 1, 0],
        [13, 2, 0, 1, 0],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
        [13, 2, 0, 0, 1],
    ],
    &[
        [1, 0, 0, 0, 0],
        [13, 2, 1, 0, 0],
        [12, 2, 1, 1, 0],
        [11, 2, 1, 1, 1],
        [12, 2, 1, 0, 1],
        [12, 2, 1, 0, 1],
        [12, 2, 1, 0, 1],
        [12, 2, 1, 0, 1],
        [12, 2, 1, 0, 1],
        [12, 2, 1, 0, 1],
    ],
    &[
        [1, 0, 0, 0, 0],
        [10, 4, 2, 0, 0],
        [10, 4, 1, 1, 0],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
        [8, 4, 1, 2, 1],
    ],
    &[
        [1, 0, 0, 0, 0],
        [11, 4, 1, 0, 0],
        [11, 4, 1, 0, 0],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
        [7, 4, 2, 2, 1],
    ],
];

pub const ARTIFACT_POINT_VALUES: &[u64] = &[0, 5000, 20000, 240000, 3000000, 20000000];

pub const PLANET_LEVEL_THRESHOLDS: &[u32] = &[
    16777216, 4194292, 1048561, 262128, 65520, 16368, 4080, 1008, 240, 48,
];

pub const PLANET_LEVEL_MIN: u64 = 0;
pub const PLANET_LEVEL_MAX: u64 = 9;
