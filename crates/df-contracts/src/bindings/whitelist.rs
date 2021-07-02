pub use whitelist_mod::*;
#[allow(clippy::too_many_arguments)]
mod whitelist_mod {
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
    #[doc = "Whitelist was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WHITELIST_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"hashes\",\n        \"type\": \"bytes32[]\"\n      }\n    ],\n    \"name\": \"addKeys\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"changeAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newDrip\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeDrip\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"drip\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getNAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_admin\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"_whitelistEnabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"key\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"isKeyValid\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isWhitelisted\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"receiveEther\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"toRemove\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeFromWhitelist\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"key\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"useKey\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Whitelist<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for Whitelist<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for Whitelist<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Whitelist))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> Whitelist<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), WHITELIST_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `addKeys` (0x188caf8a) function"]
        pub fn add_keys(
            &self,
            hashes: Vec<[u8; 32]>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 140, 175, 138], hashes)
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
        #[doc = "Calls the contract's `changeDrip` (0x8d82b0a4) function"]
        pub fn change_drip(
            &self,
            new_drip: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 130, 176, 164], new_drip)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drip` (0x9f678cca) function"]
        pub fn drip(&self) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([159, 103, 140, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNAllowed` (0xc175559b) function"]
        pub fn get_n_allowed(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([193, 117, 85, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x400ada75) function"]
        pub fn initialize(
            &self,
            admin: ethers_core::types::Address,
            whitelist_enabled: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 10, 218, 117], (admin, whitelist_enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKeyValid` (0xd8d5fc87) function"]
        pub fn is_key_valid(
            &self,
            key: String,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([216, 213, 252, 135], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isWhitelisted` (0x3af32abf) function"]
        pub fn is_whitelisted(
            &self,
            addr: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([58, 243, 42, 191], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `receiveEther` (0xa3912ec8) function"]
        pub fn receive_ether(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 145, 46, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeFromWhitelist` (0x8ab1d681) function"]
        pub fn remove_from_whitelist(
            &self,
            to_remove: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 177, 214, 129], to_remove)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `useKey` (0xbadaad2a) function"]
        pub fn use_key(
            &self,
            key: String,
            owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 218, 173, 42], (key, owner))
                .expect("method not found (this should never happen)")
        }
    }
}
