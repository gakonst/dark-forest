pub use darkforeststoragev1_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforeststoragev1_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };
    #[doc = "DarkForestStorageV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTSTORAGEV1_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"name\": \"ADMIN_CAN_ADD_PLANETS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TARGET4_RADIUS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TOKEN_MINT_END_TIMESTAMP\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WORLD_RADIUS_LOCKED\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"adminAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"artifactIdToPlanetId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"artifactIdToVoyageId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"gameConstants\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"MAX_NATURAL_PLANET_LEVEL\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"TIME_FACTOR_HUNDREDTHS\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_3\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MIN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MAX\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANET_RARITY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PHOTOID_ACTIVATION_DELAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"LOCATION_REVEAL_COOLDOWN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8[5][10][4]\",\n            \"name\": \"PLANET_TYPE_WEIGHTS\",\n            \"type\": \"uint8[5][10][4]\"\n          },\n          {\n            \"internalType\": \"uint256[6]\",\n            \"name\": \"ARTIFACT_POINT_VALUES\",\n            \"type\": \"uint256[6]\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.GameConstants\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getArtifactPointValues\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[6]\",\n        \"name\": \"\",\n        \"type\": \"uint256[6]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCumulativeRarities\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getDefaultStats\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"string\",\n            \"name\": \"label\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"barbarianPercentage\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetDefaultStats[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNPlanets\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNPlayers\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNRevealedPlanets\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"arrivalId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetArrival\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetEvent\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetEventType\",\n            \"name\": \"eventType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeTrigger\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeAdded\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetEventMetadata\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetEventsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPlanetLevelThresholds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getRevealedCoords\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"x\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"y\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"revealer\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.RevealedCoords\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getTypeWeights\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8[5][10][4]\",\n        \"name\": \"\",\n        \"type\": \"uint8[5][10][4]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getUpgrades\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popCapMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popGroMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"rangeMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speedMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defMultiplier\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Upgrade[4][3]\",\n        \"name\": \"\",\n        \"type\": \"tuple[4][3]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetArrivals\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetArtifacts\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"level\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetDefaultStats\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"string\",\n            \"name\": \"label\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"barbarianPercentage\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetDefaultStats\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetEvents\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetEventType\",\n            \"name\": \"eventType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeTrigger\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeAdded\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetEventMetadata[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"planetEventsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"planetLevelsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planets\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"population\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isHomePlanet\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Planet\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetsExtendedInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"createdAt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastUpdated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.SpaceType\",\n            \"name\": \"spaceType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState0\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"hatLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasTriedFindingArtifact\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"prospectedBlockNumber\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"destroyed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"playerIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"key\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"players\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"initTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"homePlanetId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastRevealTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawnSilver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalArtifactPoints\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Player\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revealedCoords\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"x\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"y\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"revealer\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.RevealedCoords\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revealedPlanetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"s\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"adminAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract Whitelist\",\n        \"name\": \"whitelist\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract DarkForestTokens\",\n        \"name\": \"tokens\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"paused\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"ADMIN_CAN_ADD_PLANETS\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"WORLD_RADIUS_LOCKED\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"TOKEN_MINT_END_TIMESTAMP\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"TARGET4_RADIUS\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"DISABLE_ZK_CHECKS\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANETHASH_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"SPACETYPE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOMEBASE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_X\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_Y\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_LENGTH_SCALE\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.SnarkConstants\",\n        \"name\": \"snarkConstants\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"MAX_NATURAL_PLANET_LEVEL\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"TIME_FACTOR_HUNDREDTHS\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_3\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MIN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MAX\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANET_RARITY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PHOTOID_ACTIVATION_DELAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"LOCATION_REVEAL_COOLDOWN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8[5][10][4]\",\n            \"name\": \"PLANET_TYPE_WEIGHTS\",\n            \"type\": \"uint8[5][10][4]\"\n          },\n          {\n            \"internalType\": \"uint256[6]\",\n            \"name\": \"ARTIFACT_POINT_VALUES\",\n            \"type\": \"uint256[6]\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.GameConstants\",\n        \"name\": \"gameConstants\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"planetLevelsCount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"worldRadius\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"planetEventsCount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"snarkConstants\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"DISABLE_ZK_CHECKS\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANETHASH_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"SPACETYPE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOMEBASE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_X\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_Y\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_LENGTH_SCALE\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.SnarkConstants\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"worldRadius\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DarkForestStorageV1<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestStorageV1<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestStorageV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestStorageV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestStorageV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                DARKFORESTSTORAGEV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `ADMIN_CAN_ADD_PLANETS` (0xe4045d6d) function"]
        pub fn admin_can_add_planets(&self) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 4, 93, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TARGET4_RADIUS` (0x110299fe) function"]
        pub fn target4_radius(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([17, 2, 153, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TOKEN_MINT_END_TIMESTAMP` (0x13831479) function"]
        pub fn token_mint_end_timestamp(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([19, 131, 20, 121], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WORLD_RADIUS_LOCKED` (0x421fe477) function"]
        pub fn world_radius_locked(&self) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 31, 228, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminAddress` (0xfc6f9468) function"]
        pub fn admin_address(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([252, 111, 148, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `artifactIdToPlanetId` (0xda40b311) function"]
        pub fn artifact_id_to_planet_id(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([218, 64, 179, 17], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `artifactIdToVoyageId` (0x1a3971c6) function"]
        pub fn artifact_id_to_voyage_id(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([26, 57, 113, 198], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gameConstants` (0xb60cb3b6) function"]
        pub fn game_constants(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                [[[u8; 5]; 10]; 4],
                [ethers_core::types::U256; 6],
            ),
        > {
            self.0
                .method_hash([182, 12, 179, 182], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getArtifactPointValues` (0xbdc56af5) function"]
        pub fn get_artifact_point_values(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, [ethers_core::types::U256; 6]> {
            self.0
                .method_hash([189, 197, 106, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCumulativeRarities` (0x63ffb9ee) function"]
        pub fn get_cumulative_rarities(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([99, 255, 185, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDefaultStats` (0x08d2e95a) function"]
        pub fn get_default_stats(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                String,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([8, 210, 233, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNPlanets` (0xc68b8126) function"]
        pub fn get_n_planets(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([198, 139, 129, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNPlayers` (0x8a05b329) function"]
        pub fn get_n_players(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([138, 5, 179, 41], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNRevealedPlanets` (0xd4cc63e7) function"]
        pub fn get_n_revealed_planets(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([212, 204, 99, 231], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPlanetArrival` (0x52b07fb8) function"]
        pub fn get_planet_arrival(
            &self,
            arrival_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u8,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([82, 176, 127, 184], arrival_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPlanetEvent` (0xdac40ae7) function"]
        pub fn get_planet_event(
            &self,
            location_id: ethers_core::types::U256,
            idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                u8,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([218, 196, 10, 231], (location_id, idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPlanetEventsCount` (0x3ee52a91) function"]
        pub fn get_planet_events_count(
            &self,
            location_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([62, 229, 42, 145], location_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPlanetLevelThresholds` (0x05b86be5) function"]
        pub fn get_planet_level_thresholds(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([5, 184, 107, 229], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRevealedCoords` (0x0e1d2115) function"]
        pub fn get_revealed_coords(
            &self,
            location_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::Address,
            ),
        > {
            self.0
                .method_hash([14, 29, 33, 21], location_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTypeWeights` (0x19519983) function"]
        pub fn get_type_weights(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, [[[u8; 5]; 10]; 4]> {
            self.0
                .method_hash([25, 81, 153, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUpgrades` (0x695bd00c) function"]
        pub fn get_upgrades(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            [[(
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ); 4]; 3],
        > {
            self.0
                .method_hash([105, 91, 208, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetArrivals` (0x76af38b4) function"]
        pub fn planet_arrivals(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u8,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([118, 175, 56, 180], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetArtifacts` (0xd3a3f551) function"]
        pub fn planet_artifacts(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([211, 163, 245, 81], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetDefaultStats` (0xb5549e64) function"]
        pub fn planet_default_stats(
            &self,
            level: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                String,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([181, 84, 158, 100], level)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetEvents` (0x5633aacd) function"]
        pub fn planet_events(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                ethers_core::types::U256,
                u8,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([86, 51, 170, 205], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetEventsCount` (0xbb4908f2) function"]
        pub fn planet_events_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([187, 73, 8, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetIds` (0xad489091) function"]
        pub fn planet_ids(
            &self,
            idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([173, 72, 144, 145], idx)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetLevelsCount` (0xfb9b90c2) function"]
        pub fn planet_levels_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([251, 155, 144, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planets` (0x26c1e750) function"]
        pub fn planets(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u8,
                bool,
            ),
        > {
            self.0
                .method_hash([38, 193, 231, 80], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `planetsExtendedInfo` (0xccf738e9) function"]
        pub fn planets_extended_info(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u8,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                bool,
                ethers_core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([204, 247, 56, 233], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `playerIds` (0xc012b112) function"]
        pub fn player_ids(
            &self,
            idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([192, 18, 177, 18], idx)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `players` (0xe2eb41ff) function"]
        pub fn players(
            &self,
            key: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                bool,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([226, 235, 65, 255], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revealedCoords` (0xedb7ffe4) function"]
        pub fn revealed_coords(
            &self,
            key: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::Address,
            ),
        > {
            self.0
                .method_hash([237, 183, 255, 228], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revealedPlanetIds` (0x2a9b1a32) function"]
        pub fn revealed_planet_ids(
            &self,
            idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([42, 155, 26, 50], idx)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `s` (0x86b714e2) function"]
        pub fn s(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::Address,
                bool,
                bool,
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                (
                    bool,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    bool,
                    bool,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    [[[u8; 5]; 10]; 4],
                    [ethers_core::types::U256; 6],
                ),
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([134, 183, 20, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `snarkConstants` (0x51baf19e) function"]
        pub fn snark_constants(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                bool,
                bool,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([81, 186, 241, 158], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `worldRadius` (0x4eb0e959) function"]
        pub fn world_radius(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([78, 176, 233, 89], ())
                .expect("method not found (this should never happen)")
        }
    }
}
