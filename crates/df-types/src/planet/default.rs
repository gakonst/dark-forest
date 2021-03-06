use ethers::types::U256;
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The default stats for planets
/// https://github.com/darkforest-eth/eth/blob/dfee25a1940f652f3025ac1f742c5125d350730a/contracts/DarkForestInitialize.sol#L23-L165
pub struct PlanetDefaultStats {
    pub label: &'static str,
    pub population_cap: U256,
    pub population_growth: U256,
    pub range: U256,
    pub speed: U256,
    pub defense: U256,
    pub silver_growth: U256,
    pub silver_cap: U256,
    pub barbarian_percentage: U256,
}

/// Pre-instantiated default stats for all planet levels
pub static DEFAULTS: Lazy<Vec<PlanetDefaultStats>> = Lazy::new(|| {
    vec![
        PlanetDefaultStats {
            label: "Asteroid",
            population_cap: 100000.into(),
            population_growth: 417.into(),
            range: 99.into(),
            speed: 75.into(),
            defense: 400.into(),
            silver_growth: 0.into(),
            silver_cap: 0.into(),
            barbarian_percentage: 0.into(),
        },
        PlanetDefaultStats {
            label: "Brown Dwarf",
            population_cap: 400000.into(),
            population_growth: 833.into(),
            range: 177.into(),
            speed: 75.into(),
            defense: 400.into(),
            silver_growth: 56.into(),
            silver_cap: 100000.into(),
            barbarian_percentage: 1.into(),
        },
        PlanetDefaultStats {
            label: "Red Dwarf",
            population_cap: 1600000.into(),
            population_growth: 1250.into(),
            range: 315.into(),
            speed: 75.into(),
            defense: 300.into(),
            silver_growth: 167.into(),
            silver_cap: 500000.into(),
            barbarian_percentage: 2.into(),
        },
        PlanetDefaultStats {
            label: "White Dwarf",
            population_cap: 6000000.into(),
            population_growth: 1667.into(),
            range: 591.into(),
            speed: 75.into(),
            defense: 300.into(),
            silver_growth: 417.into(),
            silver_cap: 2500000.into(),
            barbarian_percentage: 3.into(),
        },
        PlanetDefaultStats {
            label: "Yellow Star",
            population_cap: 25000000.into(),
            population_growth: 2083.into(),
            range: 1025.into(),
            speed: 75.into(),
            defense: 300.into(),
            silver_growth: 833.into(),
            silver_cap: 12000000.into(),
            barbarian_percentage: 4.into(),
        },
        PlanetDefaultStats {
            label: "Blue Star",
            population_cap: 100000000.into(),
            population_growth: 2500.into(),
            range: 1734.into(),
            speed: 75.into(),
            defense: 200.into(),
            silver_growth: 1667.into(),
            silver_cap: 50000000.into(),
            barbarian_percentage: 5.into(),
        },
        PlanetDefaultStats {
            label: "Giant",
            population_cap: 300000000.into(),
            population_growth: 2917.into(),
            range: 2838.into(),
            speed: 75.into(),
            defense: 200.into(),
            silver_growth: 2778.into(),
            silver_cap: 100000000.into(),
            barbarian_percentage: 7.into(),
        },
        PlanetDefaultStats {
            label: "Supergiant",
            population_cap: 500000000.into(),
            population_growth: 3333.into(),
            range: 4414.into(),
            speed: 75.into(),
            defense: 200.into(),
            silver_growth: 2778.into(),
            silver_cap: 200000000.into(),
            barbarian_percentage: 10.into(),
        },
        PlanetDefaultStats {
            label: "Unlabeled1",
            population_cap: 700000000.into(),
            population_growth: 3750.into(),
            range: 6306.into(),
            speed: 75.into(),
            defense: 200.into(),
            silver_growth: 2778.into(),
            silver_cap: 300000000.into(),
            barbarian_percentage: 20.into(),
        },
        PlanetDefaultStats {
            label: "Unlabeled2",
            population_cap: 800000000.into(),
            population_growth: 4167.into(),
            range: 8829.into(),
            speed: 75.into(),
            defense: 200.into(),
            silver_growth: 2778.into(),
            silver_cap: 400000000.into(),
            barbarian_percentage: 25.into(),
        },
    ]
});
