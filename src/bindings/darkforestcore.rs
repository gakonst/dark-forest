pub use darkforestcore_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforestcore_mod {
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
    #[doc = "DarkForestCore was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTCORE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"arrivalId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"from\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"to\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArrivalQueued\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArtifactActivated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArtifactDeactivated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArtifactDeposited\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArtifactFound\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ArtifactWithdrawn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"revealer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LocationRevealed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"tohatLevel\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PlanetHatBought\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PlanetProspected\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PlanetSilverWithdrawn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"receiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"PlanetTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"branch\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"toBranchLevel\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PlanetUpgraded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loc\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"PlayerInitialized\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ADMIN_CAN_ADD_PLANETS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TARGET4_RADIUS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TOKEN_MINT_END_TIMESTAMP\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WORLD_RADIUS_LOCKED\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"wormholeTo\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"activateArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"adminAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_newRadius\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"adminSetWorldRadius\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"artifactIdToPlanetId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"artifactIdToVoyageId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_location\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"buyHat\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"changeAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newCooldown\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeLocationRevealCooldown\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_newConstant\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeTarget4RadiusConstant\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"_b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[9]\",\n        \"name\": \"_input\",\n        \"type\": \"uint256[9]\"\n      }\n    ],\n    \"name\": \"checkRevealProof\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"location\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"level\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"requireValidLocationId\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.AdminCreatePlanetArgs\",\n        \"name\": \"args\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"createPlanet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"deactivateArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"depositArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"_b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[7]\",\n        \"name\": \"_input\",\n        \"type\": \"uint256[7]\"\n      }\n    ],\n    \"name\": \"findArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"gameConstants\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"MAX_NATURAL_PLANET_LEVEL\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"TIME_FACTOR_HUNDREDTHS\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_3\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MIN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MAX\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANET_RARITY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PHOTOID_ACTIVATION_DELAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"LOCATION_REVEAL_COOLDOWN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8[5][10][4]\",\n            \"name\": \"PLANET_TYPE_WEIGHTS\",\n            \"type\": \"uint8[5][10][4]\"\n          },\n          {\n            \"internalType\": \"uint256[6]\",\n            \"name\": \"ARTIFACT_POINT_VALUES\",\n            \"type\": \"uint256[6]\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.GameConstants\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getArtifactPointValues\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[6]\",\n        \"name\": \"\",\n        \"type\": \"uint256[6]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCumulativeRarities\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getDefaultStats\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"string\",\n            \"name\": \"label\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"barbarianPercentage\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetDefaultStats[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNPlanets\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNPlayers\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNRevealedPlanets\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"arrivalId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetArrival\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetEvent\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetEventType\",\n            \"name\": \"eventType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeTrigger\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeAdded\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetEventMetadata\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetEventsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPlanetLevelThresholds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"location\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"timestamp\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getRefreshedPlanet\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"population\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isHomePlanet\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Planet\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"createdAt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastUpdated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.SpaceType\",\n            \"name\": \"spaceType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState0\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"hatLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasTriedFindingArtifact\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"prospectedBlockNumber\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"destroyed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"uint256[12]\",\n        \"name\": \"eventsToRemove\",\n        \"type\": \"uint256[12]\"\n      },\n      {\n        \"internalType\": \"uint256[12]\",\n        \"name\": \"artifactsToAdd\",\n        \"type\": \"uint256[12]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getRevealedCoords\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"x\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"y\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"revealer\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.RevealedCoords\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getTypeWeights\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8[5][10][4]\",\n        \"name\": \"\",\n        \"type\": \"uint8[5][10][4]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getUpgrades\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popCapMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popGroMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"rangeMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speedMultiplier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defMultiplier\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Upgrade[4][3]\",\n        \"name\": \"\",\n        \"type\": \"tuple[4][3]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"_b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[8]\",\n        \"name\": \"_input\",\n        \"type\": \"uint256[8]\"\n      }\n    ],\n    \"name\": \"initializePlayer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"_b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[13]\",\n        \"name\": \"_input\",\n        \"type\": \"uint256[13]\"\n      }\n    ],\n    \"name\": \"move\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetArrivals\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetArtifacts\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"level\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetDefaultStats\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"string\",\n            \"name\": \"label\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"barbarianPercentage\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetDefaultStats\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetEvents\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetEventType\",\n            \"name\": \"eventType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeTrigger\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeAdded\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetEventMetadata[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"planetEventsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"planetLevelsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planets\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"population\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isHomePlanet\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Planet\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"planetsExtendedInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"createdAt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastUpdated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.SpaceType\",\n            \"name\": \"spaceType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState0\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"hatLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasTriedFindingArtifact\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"prospectedBlockNumber\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"destroyed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"playerIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"key\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"players\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"initTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"homePlanetId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastRevealTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawnSilver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalArtifactPoints\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Player\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"prospectPlanet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"location\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"refreshPlanet\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"_b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"_c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[9]\",\n        \"name\": \"_input\",\n        \"type\": \"uint256[9]\"\n      }\n    ],\n    \"name\": \"revealLocation\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"key\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revealedCoords\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"x\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"y\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"revealer\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.RevealedCoords\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revealedPlanetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"s\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"adminAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract Whitelist\",\n        \"name\": \"whitelist\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract DarkForestTokens\",\n        \"name\": \"tokens\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"paused\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"ADMIN_CAN_ADD_PLANETS\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"WORLD_RADIUS_LOCKED\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"TOKEN_MINT_END_TIMESTAMP\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"TARGET4_RADIUS\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"DISABLE_ZK_CHECKS\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANETHASH_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"SPACETYPE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOMEBASE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_X\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_Y\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_LENGTH_SCALE\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.SnarkConstants\",\n        \"name\": \"snarkConstants\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"MAX_NATURAL_PLANET_LEVEL\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"TIME_FACTOR_HUNDREDTHS\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_THRESHOLD_3\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MIN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"INIT_PERLIN_MAX\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOME_THRESHOLD_2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANET_RARITY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PHOTOID_ACTIVATION_DELAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"LOCATION_REVEAL_COOLDOWN\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8[5][10][4]\",\n            \"name\": \"PLANET_TYPE_WEIGHTS\",\n            \"type\": \"uint8[5][10][4]\"\n          },\n          {\n            \"internalType\": \"uint256[6]\",\n            \"name\": \"ARTIFACT_POINT_VALUES\",\n            \"type\": \"uint256[6]\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.GameConstants\",\n        \"name\": \"gameConstants\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"planetLevelsCount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"worldRadius\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"planetEventsCount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newTokenMintEndTime\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setTokenMintEndTime\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"snarkConstants\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"DISABLE_ZK_CHECKS\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PLANETHASH_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"SPACETYPE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"BIOMEBASE_KEY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_X\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"PERLIN_MIRROR_Y\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"PERLIN_LENGTH_SCALE\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.SnarkConstants\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_location\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_player\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"unpause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_location\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_branch\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"upgradePlanet\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdrawArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdrawSilver\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"worldRadius\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DarkForestCore<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestCore<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestCore<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestCore))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestCore<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), DARKFORESTCORE_ABI.clone(), client);
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
        #[doc = "Calls the contract's `activateArtifact` (0x9513a2ad) function"]
        pub fn activate_artifact(
            &self,
            location_id: ethers_core::types::U256,
            artifact_id: ethers_core::types::U256,
            wormhole_to: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 19, 162, 173], (location_id, artifact_id, wormhole_to))
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
        #[doc = "Calls the contract's `adminSetWorldRadius` (0x50e1ade3) function"]
        pub fn admin_set_world_radius(
            &self,
            new_radius: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 225, 173, 227], new_radius)
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
        #[doc = "Calls the contract's `buyHat` (0x9fd4d4d4) function"]
        pub fn buy_hat(
            &self,
            location: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 212, 212, 212], location)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeAdmin` (0x8f283970) function"]
        pub fn change_admin(
            &self,
            new_admin: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 40, 57, 112], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeLocationRevealCooldown` (0x02531a47) function"]
        pub fn change_location_reveal_cooldown(
            &self,
            new_cooldown: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 83, 26, 71], new_cooldown)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeTarget4RadiusConstant` (0x0767751a) function"]
        pub fn change_target_4_radius_constant(
            &self,
            new_constant: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 103, 117, 26], new_constant)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkRevealProof` (0x49105550) function"]
        pub fn check_reveal_proof(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 9],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 16, 85, 80], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createPlanet` (0xbcb324d8) function"]
        pub fn create_planet(
            &self,
            args: (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u8,
                bool,
            ),
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 179, 36, 216], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deactivateArtifact` (0xc918aebd) function"]
        pub fn deactivate_artifact(
            &self,
            location_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 24, 174, 189], location_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositArtifact` (0x92d93a12) function"]
        pub fn deposit_artifact(
            &self,
            location_id: ethers_core::types::U256,
            artifact_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 217, 58, 18], (location_id, artifact_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `findArtifact` (0x98d042f2) function"]
        pub fn find_artifact(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 7],
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 208, 66, 242], (a, b, c, input))
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
        #[doc = "Calls the contract's `getRefreshedPlanet` (0x8f0742bf) function"]
        pub fn get_refreshed_planet(
            &self,
            location: ethers_core::types::U256,
            timestamp: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
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
                [ethers_core::types::U256; 12],
                [ethers_core::types::U256; 12],
            ),
        > {
            self.0
                .method_hash([143, 7, 66, 191], (location, timestamp))
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
        #[doc = "Calls the contract's `initializePlayer` (0xd161f02c) function"]
        pub fn initialize_player(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 8],
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([209, 97, 240, 44], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `move` (0x5f212908) function"]
        pub fn move_(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 13],
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([95, 33, 41, 8], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
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
        #[doc = "Calls the contract's `prospectPlanet` (0xe76aa429) function"]
        pub fn prospect_planet(
            &self,
            location_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 106, 164, 41], location_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refreshPlanet` (0x3f324cf3) function"]
        pub fn refresh_planet(
            &self,
            location: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 50, 76, 243], location)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revealLocation` (0xa44751a7) function"]
        pub fn reveal_location(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 9],
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([164, 71, 81, 167], (a, b, c, input))
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
        #[doc = "Calls the contract's `setTokenMintEndTime` (0xea8046a0) function"]
        pub fn set_token_mint_end_time(
            &self,
            new_token_mint_end_time: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 128, 70, 160], new_token_mint_end_time)
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
        #[doc = "Calls the contract's `transferOwnership` (0x29507f73) function"]
        pub fn transfer_ownership(
            &self,
            location: ethers_core::types::U256,
            player: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 80, 127, 115], (location, player))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradePlanet` (0x73a85381) function"]
        pub fn upgrade_planet(
            &self,
            location: ethers_core::types::U256,
            branch: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, ethers_core::types::U256),
        > {
            self.0
                .method_hash([115, 168, 83, 129], (location, branch))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x3ccfd60b) function"]
        pub fn withdraw(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawArtifact` (0x70e0f0ce) function"]
        pub fn withdraw_artifact(
            &self,
            location_id: ethers_core::types::U256,
            artifact_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 224, 240, 206], (location_id, artifact_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawSilver` (0x443a397c) function"]
        pub fn withdraw_silver(
            &self,
            location_id: ethers_core::types::U256,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 58, 57, 124], (location_id, amount))
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
        #[doc = "Gets the contract's `ArrivalQueued` event"]
        pub fn arrival_queued_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArrivalQueuedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtifactActivated` event"]
        pub fn artifact_activated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArtifactActivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtifactDeactivated` event"]
        pub fn artifact_deactivated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArtifactDeactivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtifactDeposited` event"]
        pub fn artifact_deposited_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArtifactDepositedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtifactFound` event"]
        pub fn artifact_found_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArtifactFoundFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtifactWithdrawn` event"]
        pub fn artifact_withdrawn_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ArtifactWithdrawnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LocationRevealed` event"]
        pub fn location_revealed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LocationRevealedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlanetHatBought` event"]
        pub fn planet_hat_bought_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlanetHatBoughtFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlanetProspected` event"]
        pub fn planet_prospected_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlanetProspectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlanetSilverWithdrawn` event"]
        pub fn planet_silver_withdrawn_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlanetSilverWithdrawnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlanetTransferred` event"]
        pub fn planet_transferred_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlanetTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlanetUpgraded` event"]
        pub fn planet_upgraded_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlanetUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlayerInitialized` event"]
        pub fn player_initialized_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PlayerInitializedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, DarkForestCoreEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ArrivalQueued",
        abi = "ArrivalQueued(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct ArrivalQueuedFilter {
        pub player: ethers_core::types::Address,
        pub arrival_id: ethers_core::types::U256,
        pub from: ethers_core::types::U256,
        pub to: ethers_core::types::U256,
        pub artifact_id: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ArtifactActivated",
        abi = "ArtifactActivated(address,uint256,uint256)"
    )]
    pub struct ArtifactActivatedFilter {
        pub player: ethers_core::types::Address,
        pub artifact_id: ethers_core::types::U256,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ArtifactDeactivated",
        abi = "ArtifactDeactivated(address,uint256,uint256)"
    )]
    pub struct ArtifactDeactivatedFilter {
        pub player: ethers_core::types::Address,
        pub artifact_id: ethers_core::types::U256,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ArtifactDeposited",
        abi = "ArtifactDeposited(address,uint256,uint256)"
    )]
    pub struct ArtifactDepositedFilter {
        pub player: ethers_core::types::Address,
        pub artifact_id: ethers_core::types::U256,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ArtifactFound", abi = "ArtifactFound(address,uint256,uint256)")]
    pub struct ArtifactFoundFilter {
        pub player: ethers_core::types::Address,
        pub artifact_id: ethers_core::types::U256,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ArtifactWithdrawn",
        abi = "ArtifactWithdrawn(address,uint256,uint256)"
    )]
    pub struct ArtifactWithdrawnFilter {
        pub player: ethers_core::types::Address,
        pub artifact_id: ethers_core::types::U256,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "LocationRevealed", abi = "LocationRevealed(address,uint256)")]
    pub struct LocationRevealedFilter {
        pub revealer: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "PlanetHatBought",
        abi = "PlanetHatBought(address,uint256,uint256)"
    )]
    pub struct PlanetHatBoughtFilter {
        pub player: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
        pub tohat_level: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "PlanetProspected", abi = "PlanetProspected(address,uint256)")]
    pub struct PlanetProspectedFilter {
        pub player: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "PlanetSilverWithdrawn",
        abi = "PlanetSilverWithdrawn(address,uint256,uint256)"
    )]
    pub struct PlanetSilverWithdrawnFilter {
        pub player: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "PlanetTransferred",
        abi = "PlanetTransferred(address,uint256,address)"
    )]
    pub struct PlanetTransferredFilter {
        pub sender: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
        pub receiver: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "PlanetUpgraded",
        abi = "PlanetUpgraded(address,uint256,uint256,uint256)"
    )]
    pub struct PlanetUpgradedFilter {
        pub player: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
        pub branch: ethers_core::types::U256,
        pub to_branch_level: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "PlayerInitialized", abi = "PlayerInitialized(address,uint256)")]
    pub struct PlayerInitializedFilter {
        pub player: ethers_core::types::Address,
        pub loc: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DarkForestCoreEvents {
        ArrivalQueuedFilter(ArrivalQueuedFilter),
        ArtifactActivatedFilter(ArtifactActivatedFilter),
        ArtifactDeactivatedFilter(ArtifactDeactivatedFilter),
        ArtifactDepositedFilter(ArtifactDepositedFilter),
        ArtifactFoundFilter(ArtifactFoundFilter),
        ArtifactWithdrawnFilter(ArtifactWithdrawnFilter),
        LocationRevealedFilter(LocationRevealedFilter),
        PlanetHatBoughtFilter(PlanetHatBoughtFilter),
        PlanetProspectedFilter(PlanetProspectedFilter),
        PlanetSilverWithdrawnFilter(PlanetSilverWithdrawnFilter),
        PlanetTransferredFilter(PlanetTransferredFilter),
        PlanetUpgradedFilter(PlanetUpgradedFilter),
        PlayerInitializedFilter(PlayerInitializedFilter),
    }
    impl ethers_core::abi::Tokenizable for DarkForestCoreEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ArrivalQueuedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArrivalQueuedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactActivatedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArtifactActivatedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactDeactivatedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArtifactDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactDepositedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArtifactDepositedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactFoundFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArtifactFoundFilter(decoded));
            }
            if let Ok(decoded) = ArtifactWithdrawnFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::ArtifactWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = LocationRevealedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::LocationRevealedFilter(decoded));
            }
            if let Ok(decoded) = PlanetHatBoughtFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlanetHatBoughtFilter(decoded));
            }
            if let Ok(decoded) = PlanetProspectedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlanetProspectedFilter(decoded));
            }
            if let Ok(decoded) = PlanetSilverWithdrawnFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlanetSilverWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = PlanetTransferredFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlanetTransferredFilter(decoded));
            }
            if let Ok(decoded) = PlanetUpgradedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlanetUpgradedFilter(decoded));
            }
            if let Ok(decoded) = PlayerInitializedFilter::from_token(token.clone()) {
                return Ok(DarkForestCoreEvents::PlayerInitializedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                DarkForestCoreEvents::ArrivalQueuedFilter(element) => element.into_token(),
                DarkForestCoreEvents::ArtifactActivatedFilter(element) => element.into_token(),
                DarkForestCoreEvents::ArtifactDeactivatedFilter(element) => element.into_token(),
                DarkForestCoreEvents::ArtifactDepositedFilter(element) => element.into_token(),
                DarkForestCoreEvents::ArtifactFoundFilter(element) => element.into_token(),
                DarkForestCoreEvents::ArtifactWithdrawnFilter(element) => element.into_token(),
                DarkForestCoreEvents::LocationRevealedFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlanetHatBoughtFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlanetProspectedFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlanetSilverWithdrawnFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlanetTransferredFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlanetUpgradedFilter(element) => element.into_token(),
                DarkForestCoreEvents::PlayerInitializedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for DarkForestCoreEvents {}
    impl ethers_contract::EthLogDecode for DarkForestCoreEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ArrivalQueuedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArrivalQueuedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactActivatedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArtifactActivatedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactDeactivatedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArtifactDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactDepositedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArtifactDepositedFilter(decoded));
            }
            if let Ok(decoded) = ArtifactFoundFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArtifactFoundFilter(decoded));
            }
            if let Ok(decoded) = ArtifactWithdrawnFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::ArtifactWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = LocationRevealedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::LocationRevealedFilter(decoded));
            }
            if let Ok(decoded) = PlanetHatBoughtFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlanetHatBoughtFilter(decoded));
            }
            if let Ok(decoded) = PlanetProspectedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlanetProspectedFilter(decoded));
            }
            if let Ok(decoded) = PlanetSilverWithdrawnFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlanetSilverWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = PlanetTransferredFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlanetTransferredFilter(decoded));
            }
            if let Ok(decoded) = PlanetUpgradedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlanetUpgradedFilter(decoded));
            }
            if let Ok(decoded) = PlayerInitializedFilter::decode_log(log) {
                return Ok(DarkForestCoreEvents::PlayerInitializedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
