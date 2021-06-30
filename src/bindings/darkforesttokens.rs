pub use darkforesttokens_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforesttokens_mod {
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
    #[doc = "DarkForestTokens was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTTOKENS_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"approved\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"operator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"approved\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"ApprovalForAll\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"baseURI\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_newAdminAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"changeAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"tokenId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"discoverer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetId\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n            \"name\": \"rarity\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.Biome\",\n            \"name\": \"biome\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n            \"name\": \"artifactType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.DFTCreateArtifactArgs\",\n        \"name\": \"args\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"createArtifact\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetDiscoveredOn\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n            \"name\": \"rarity\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.Biome\",\n            \"name\": \"planetBiome\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"mintedAtTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"discoverer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n            \"name\": \"artifactType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastActivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastDeactivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"wormholeTo\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Artifact\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"doesArtifactExist\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getApproved\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getArtifact\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetDiscoveredOn\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n            \"name\": \"rarity\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.Biome\",\n            \"name\": \"planetBiome\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"mintedAtTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"discoverer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n            \"name\": \"artifactType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastActivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastDeactivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"wormholeTo\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Artifact\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"playerId\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getPlayerArtifactIds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_coreAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_adminAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"operator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isApprovedForAll\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ownerOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"safeTransferFrom\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"safeTransferFrom\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"operator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"approved\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setApprovalForAll\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"interfaceId\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"supportsInterface\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"tokenByIndex\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"tokenOfOwnerByIndex\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"tokenURI\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isInitialized\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"id\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"planetDiscoveredOn\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactRarity\",\n            \"name\": \"rarity\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.Biome\",\n            \"name\": \"planetBiome\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"mintedAtTimestamp\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"discoverer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"enum DarkForestTypes.ArtifactType\",\n            \"name\": \"artifactType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastActivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"lastDeactivated\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"wormholeTo\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DarkForestTypes.Artifact\",\n        \"name\": \"updatedArtifact\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"updateArtifact\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DarkForestTokens<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestTokens<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestTokens<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestTokens))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestTokens<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                DARKFORESTTOKENS_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers_core::types::Address,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `baseURI` (0x6c0360eb) function"]
        pub fn base_uri(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([108, 3, 96, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeAdmin` (0x8f283970) function"]
        pub fn change_admin(
            &self,
            new_admin_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 40, 57, 112], new_admin_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createArtifact` (0xc1a70e6c) function"]
        pub fn create_artifact(
            &self,
            args: (
                ethers_core::types::U256,
                ethers_core::types::Address,
                ethers_core::types::U256,
                u8,
                u8,
                u8,
                ethers_core::types::Address,
            ),
        ) -> ethers_contract::builders::ContractCall<
            M,
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
        > {
            self.0
                .method_hash([193, 167, 14, 108], args)
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
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getArtifact` (0x8465ad44) function"]
        pub fn get_artifact(
            &self,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
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
        > {
            self.0
                .method_hash([132, 101, 173, 68], token_id)
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
        #[doc = "Calls the contract's `initialize` (0x485cc955) function"]
        pub fn initialize(
            &self,
            core_address: ethers_core::types::Address,
            admin_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (core_address, admin_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers_core::types::Address,
            operator: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_data(
            &self,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            token_id: ethers_core::types::U256,
            data: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers_core::types::Address,
            approved: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenByIndex` (0x4f6ccce7) function"]
        pub fn token_by_index(
            &self,
            index: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function"]
        pub fn token_of_owner_by_index(
            &self,
            owner: ethers_core::types::Address,
            index: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferArtifact` (0x991182da) function"]
        pub fn transfer_artifact(
            &self,
            token_id: ethers_core::types::U256,
            new_owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 17, 130, 218], (token_id, new_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            token_id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateArtifact` (0x868e2810) function"]
        pub fn update_artifact(
            &self,
            updated_artifact: (
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
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 142, 40, 16], updated_artifact)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers_contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers_contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, DarkForestTokensEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub approved: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers_core::types::Address,
        pub approved: bool,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DarkForestTokensEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers_core::abi::Tokenizable for DarkForestTokensEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::from_token(token.clone()) {
                return Ok(DarkForestTokensEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::from_token(token.clone()) {
                return Ok(DarkForestTokensEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::from_token(token.clone()) {
                return Ok(DarkForestTokensEvents::TransferFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                DarkForestTokensEvents::ApprovalFilter(element) => element.into_token(),
                DarkForestTokensEvents::ApprovalForAllFilter(element) => element.into_token(),
                DarkForestTokensEvents::TransferFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for DarkForestTokensEvents {}
    impl ethers_contract::EthLogDecode for DarkForestTokensEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DarkForestTokensEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(DarkForestTokensEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DarkForestTokensEvents::TransferFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
