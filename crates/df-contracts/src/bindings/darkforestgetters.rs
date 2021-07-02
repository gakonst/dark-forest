pub use darkforestgetters_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforestgetters_mod {
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
    #[doc = "DarkForestGetters was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTGETTERS_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetArtifactsByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isInitialized\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"id\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"planetDiscoveredOn\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n                \"name\": \"rarity\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.Biome\",\n                \"name\": \"planetBiome\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"mintedAtTimestamp\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"discoverer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n                \"name\": \"artifactType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastActivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastDeactivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"wormholeTo\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Artifact\",\n            \"name\": \"artifact\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"upgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"timeDelayedUpgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"voyageId\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArtifactWithMetadata[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetArrivals\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData[][]\",\n        \"name\": \"\",\n        \"type\": \"tuple[][]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetArrivalsByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData[][]\",\n        \"name\": \"\",\n        \"type\": \"tuple[][]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"planetIds\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetArtifacts\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isInitialized\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"id\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"planetDiscoveredOn\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n                \"name\": \"rarity\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.Biome\",\n                \"name\": \"planetBiome\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"mintedAtTimestamp\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"discoverer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n                \"name\": \"artifactType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastActivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastDeactivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"wormholeTo\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Artifact\",\n            \"name\": \"artifact\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"upgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"timeDelayedUpgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"voyageId\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArtifactWithMetadata[][]\",\n        \"name\": \"\",\n        \"type\": \"tuple[][]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ret\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlanets\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"population\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isHomePlanet\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Planet[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetsByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"population\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetType\",\n            \"name\": \"planetType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isHomePlanet\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Planet[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetsDataByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"range\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speed\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defense\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"population\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"populationCap\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"populationGrowth\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"silverCap\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"silverGrowth\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"silver\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"planetLevel\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.PlanetType\",\n                \"name\": \"planetType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isHomePlanet\",\n                \"type\": \"bool\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Planet\",\n            \"name\": \"planet\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isInitialized\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"createdAt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastUpdated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"perlin\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.SpaceType\",\n                \"name\": \"spaceType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"upgradeState0\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"upgradeState1\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"upgradeState2\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"hatLevel\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"hasTriedFindingArtifact\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"prospectedBlockNumber\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"destroyed\",\n                \"type\": \"bool\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo\",\n            \"name\": \"info\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"locationId\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"x\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"y\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"revealer\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.RevealedCoords\",\n            \"name\": \"revealedCoords\",\n            \"type\": \"tuple\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetData[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetsExtendedInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"createdAt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastUpdated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.SpaceType\",\n            \"name\": \"spaceType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState0\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"hatLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasTriedFindingArtifact\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"prospectedBlockNumber\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"destroyed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetPlanetsExtendedInfoByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"createdAt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastUpdated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"perlin\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.SpaceType\",\n            \"name\": \"spaceType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState0\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState1\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"upgradeState2\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"hatLevel\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasTriedFindingArtifact\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"prospectedBlockNumber\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"destroyed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetExtendedInfo[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlayerIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"ret\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetPlayers\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"initTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"homePlanetId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastRevealTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawnSilver\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalArtifactPoints\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Player[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetRevealedCoordsByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"x\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"y\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"revealer\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.RevealedCoords[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startIdx\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"endIdx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"bulkGetRevealedPlanetIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ret\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"ids\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"bulkGetVoyagesByIds\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"doesArtifactExist\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"artifactId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getArtifactById\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isInitialized\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"id\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"planetDiscoveredOn\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n                \"name\": \"rarity\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.Biome\",\n                \"name\": \"planetBiome\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"mintedAtTimestamp\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"discoverer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n                \"name\": \"artifactType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastActivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastDeactivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"wormholeTo\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Artifact\",\n            \"name\": \"artifact\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"upgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"timeDelayedUpgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"voyageId\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArtifactWithMetadata\",\n        \"name\": \"ret\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getArtifactsOnPlanet\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isInitialized\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"id\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"planetDiscoveredOn\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n                \"name\": \"rarity\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.Biome\",\n                \"name\": \"planetBiome\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"mintedAtTimestamp\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"discoverer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n                \"name\": \"artifactType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastActivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"lastDeactivated\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"wormholeTo\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Artifact\",\n            \"name\": \"artifact\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"upgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popCapMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"popGroMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"rangeMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"speedMultiplier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"defMultiplier\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DarkForestTypes.Upgrade\",\n            \"name\": \"timeDelayedUpgrade\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locationId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"voyageId\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArtifactWithMetadata[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getDefaultStats\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"string\",\n            \"name\": \"label\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"populationGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"range\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"speed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"defense\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverGrowth\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverCap\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"barbarianPercentage\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetDefaultStats[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_location\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetArrivals\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"player\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fromPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"toPlanet\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"popArriving\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"silverMoved\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"departureTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"arrivalTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArrivalType\",\n            \"name\": \"arrivalType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"carriedArtifactId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"distance\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.ArrivalData[]\",\n        \"name\": \"ret\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"locationId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"idx\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getPlanetEvent\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.PlanetEventType\",\n            \"name\": \"eventType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeTrigger\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timeAdded\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.PlanetEventMetadata\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"playerId\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getPlayerArtifactIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_adminAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_coreContractAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_tokensAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DarkForestGetters<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestGetters<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestGetters<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestGetters))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestGetters<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                DARKFORESTGETTERS_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `bulkGetArtifactsByIds` (0xe654848a) function"]
        pub fn bulk_get_artifacts_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                (
                    bool,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    u8,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::Address,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([230, 84, 132, 138], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetArrivals` (0x40a72788) function"]
        pub fn bulk_get_planet_arrivals(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<
                Vec<(
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
                )>,
            >,
        > {
            self.0
                .method_hash([64, 167, 39, 136], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetArrivalsByIds` (0x3f9d7c49) function"]
        pub fn bulk_get_planet_arrivals_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<
                Vec<(
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
                )>,
            >,
        > {
            self.0
                .method_hash([63, 157, 124, 73], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetArtifacts` (0x54113a1e) function"]
        pub fn bulk_get_planet_artifacts(
            &self,
            planet_ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<
                Vec<(
                    (
                        bool,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        u8,
                        u8,
                        ethers_core::types::U256,
                        ethers_core::types::Address,
                        u8,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                    ),
                    (
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                    ),
                    (
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                        ethers_core::types::U256,
                    ),
                    ethers_core::types::Address,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                )>,
            >,
        > {
            self.0
                .method_hash([84, 17, 58, 30], planet_ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetIds` (0xaaca42e9) function"]
        pub fn bulk_get_planet_ids(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([170, 202, 66, 233], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanets` (0x975de613) function"]
        pub fn bulk_get_planets(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([151, 93, 230, 19], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetsByIds` (0x98f94a8e) function"]
        pub fn bulk_get_planets_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([152, 249, 74, 142], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetsDataByIds` (0x45510f22) function"]
        pub fn bulk_get_planets_data_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::Address,
                ),
            )>,
        > {
            self.0
                .method_hash([69, 81, 15, 34], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetsExtendedInfo` (0x04d74a39) function"]
        pub fn bulk_get_planets_extended_info(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([4, 215, 74, 57], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlanetsExtendedInfoByIds` (0x78622ae6) function"]
        pub fn bulk_get_planets_extended_info_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([120, 98, 42, 230], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlayerIds` (0xf6d5775a) function"]
        pub fn bulk_get_player_ids(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::Address>> {
            self.0
                .method_hash([246, 213, 119, 90], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetPlayers` (0x6c7ffeec) function"]
        pub fn bulk_get_players(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                bool,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([108, 127, 254, 236], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetRevealedCoordsByIds` (0x46d7789d) function"]
        pub fn bulk_get_revealed_coords_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::Address,
            )>,
        > {
            self.0
                .method_hash([70, 215, 120, 157], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetRevealedPlanetIds` (0xfa37ca45) function"]
        pub fn bulk_get_revealed_planet_ids(
            &self,
            start_idx: ethers_core::types::U256,
            end_idx: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([250, 55, 202, 69], (start_idx, end_idx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bulkGetVoyagesByIds` (0x745297cc) function"]
        pub fn bulk_get_voyages_by_ids(
            &self,
            ids: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([116, 82, 151, 204], ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `doesArtifactExist` (0x1f94f540) function"]
        pub fn does_artifact_exist(
            &self,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 148, 245, 64], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getArtifactById` (0xf9231d92) function"]
        pub fn get_artifact_by_id(
            &self,
            artifact_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                (
                    bool,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    u8,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::Address,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([249, 35, 29, 146], artifact_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getArtifactsOnPlanet` (0x90960189) function"]
        pub fn get_artifacts_on_planet(
            &self,
            location_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                (
                    bool,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    u8,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::Address,
                    u8,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                (
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                    ethers_core::types::U256,
                ),
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([144, 150, 1, 137], location_id)
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
        #[doc = "Calls the contract's `getPlanetArrivals` (0xdacb6135) function"]
        pub fn get_planet_arrivals(
            &self,
            location: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
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
            )>,
        > {
            self.0
                .method_hash([218, 203, 97, 53], location)
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
        #[doc = "Calls the contract's `getPlayerArtifactIds` (0x314d1ff7) function"]
        pub fn get_player_artifact_ids(
            &self,
            player_id: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([49, 77, 31, 247], player_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc0c53b8b) function"]
        pub fn initialize(
            &self,
            admin_address: ethers_core::types::Address,
            core_contract_address: ethers_core::types::Address,
            tokens_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 197, 59, 139],
                    (admin_address, core_contract_address, tokens_address),
                )
                .expect("method not found (this should never happen)")
        }
    }
}
