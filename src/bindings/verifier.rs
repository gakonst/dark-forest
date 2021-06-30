pub use verifier_mod::*;
#[allow(clippy::too_many_arguments)]
mod verifier_mod {
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
    #[doc = "Verifier was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VERIFIER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[7]\",\n        \"name\": \"input\",\n        \"type\": \"uint256[7]\"\n      }\n    ],\n    \"name\": \"verifyBiomebaseProof\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[8]\",\n        \"name\": \"input\",\n        \"type\": \"uint256[8]\"\n      }\n    ],\n    \"name\": \"verifyInitProof\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[10]\",\n        \"name\": \"input\",\n        \"type\": \"uint256[10]\"\n      }\n    ],\n    \"name\": \"verifyMoveProof\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"a\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[2][2]\",\n        \"name\": \"b\",\n        \"type\": \"uint256[2][2]\"\n      },\n      {\n        \"internalType\": \"uint256[2]\",\n        \"name\": \"c\",\n        \"type\": \"uint256[2]\"\n      },\n      {\n        \"internalType\": \"uint256[9]\",\n        \"name\": \"input\",\n        \"type\": \"uint256[9]\"\n      }\n    ],\n    \"name\": \"verifyRevealProof\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Verifier<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for Verifier<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for Verifier<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> Verifier<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), VERIFIER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `verifyBiomebaseProof` (0x9e1f3678) function"]
        pub fn verify_biomebase_proof(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 7],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([158, 31, 54, 120], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyInitProof` (0x4bdf7672) function"]
        pub fn verify_init_proof(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 8],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([75, 223, 118, 114], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyMoveProof` (0x7e0bb909) function"]
        pub fn verify_move_proof(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 10],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 11, 185, 9], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyRevealProof` (0xd20293f0) function"]
        pub fn verify_reveal_proof(
            &self,
            a: [ethers_core::types::U256; 2],
            b: [[ethers_core::types::U256; 2]; 2],
            c: [ethers_core::types::U256; 2],
            input: [ethers_core::types::U256; 9],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 2, 147, 240], (a, b, c, input))
                .expect("method not found (this should never happen)")
        }
    }
}
