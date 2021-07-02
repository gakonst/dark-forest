pub use darkforestgptcredit_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforestgptcredit_mod {
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
    #[doc = "DarkForestGPTCredit was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTGPTCREDIT_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"buyer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"cost\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"BoughtCredits\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newPrice\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ChangedCreditPrice\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"DeductedCredits\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"GiftedCredits\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"buyCredits\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_newAdmin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"changeAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newPrice\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeCreditPrice\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"creditPrice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"credits\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"decreasePlayerCredits\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"player\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"giftPlayerCredits\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_admin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"receiveEther\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DarkForestGPTCredit<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestGPTCredit<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestGPTCredit<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestGPTCredit))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestGPTCredit<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                DARKFORESTGPTCREDIT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `buyCredits` (0x0c4dfe3f) function"]
        pub fn buy_credits(
            &self,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 77, 254, 63], amount)
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
        #[doc = "Calls the contract's `changeCreditPrice` (0x48464292) function"]
        pub fn change_credit_price(
            &self,
            new_price: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 70, 66, 146], new_price)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditPrice` (0xca6358cd) function"]
        pub fn credit_price(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([202, 99, 88, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `credits` (0xfe5ff468) function"]
        pub fn credits(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([254, 95, 244, 104], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreasePlayerCredits` (0x7c37c63a) function"]
        pub fn decrease_player_credits(
            &self,
            player: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 55, 198, 58], (player, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `giftPlayerCredits` (0x4c110f12) function"]
        pub fn gift_player_credits(
            &self,
            player: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 17, 15, 18], (player, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            admin: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `receiveEther` (0xa3912ec8) function"]
        pub fn receive_ether(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 145, 46, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x3ccfd60b) function"]
        pub fn withdraw(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BoughtCredits` event"]
        pub fn bought_credits_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, BoughtCreditsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ChangedCreditPrice` event"]
        pub fn changed_credit_price_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ChangedCreditPriceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DeductedCredits` event"]
        pub fn deducted_credits_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, DeductedCreditsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GiftedCredits` event"]
        pub fn gifted_credits_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, GiftedCreditsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, DarkForestGPTCreditEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "BoughtCredits", abi = "BoughtCredits(address,uint256,uint256)")]
    pub struct BoughtCreditsFilter {
        pub buyer: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub cost: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ChangedCreditPrice", abi = "ChangedCreditPrice(uint256)")]
    pub struct ChangedCreditPriceFilter {
        pub new_price: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "DeductedCredits", abi = "DeductedCredits(address,uint256)")]
    pub struct DeductedCreditsFilter {
        pub player: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "GiftedCredits", abi = "GiftedCredits(address,uint256)")]
    pub struct GiftedCreditsFilter {
        pub player: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DarkForestGPTCreditEvents {
        BoughtCreditsFilter(BoughtCreditsFilter),
        ChangedCreditPriceFilter(ChangedCreditPriceFilter),
        DeductedCreditsFilter(DeductedCreditsFilter),
        GiftedCreditsFilter(GiftedCreditsFilter),
    }
    impl ethers_core::abi::Tokenizable for DarkForestGPTCreditEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BoughtCreditsFilter::from_token(token.clone()) {
                return Ok(DarkForestGPTCreditEvents::BoughtCreditsFilter(decoded));
            }
            if let Ok(decoded) = ChangedCreditPriceFilter::from_token(token.clone()) {
                return Ok(DarkForestGPTCreditEvents::ChangedCreditPriceFilter(decoded));
            }
            if let Ok(decoded) = DeductedCreditsFilter::from_token(token.clone()) {
                return Ok(DarkForestGPTCreditEvents::DeductedCreditsFilter(decoded));
            }
            if let Ok(decoded) = GiftedCreditsFilter::from_token(token.clone()) {
                return Ok(DarkForestGPTCreditEvents::GiftedCreditsFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                DarkForestGPTCreditEvents::BoughtCreditsFilter(element) => element.into_token(),
                DarkForestGPTCreditEvents::ChangedCreditPriceFilter(element) => {
                    element.into_token()
                }
                DarkForestGPTCreditEvents::DeductedCreditsFilter(element) => element.into_token(),
                DarkForestGPTCreditEvents::GiftedCreditsFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for DarkForestGPTCreditEvents {}
    impl ethers_contract::EthLogDecode for DarkForestGPTCreditEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BoughtCreditsFilter::decode_log(log) {
                return Ok(DarkForestGPTCreditEvents::BoughtCreditsFilter(decoded));
            }
            if let Ok(decoded) = ChangedCreditPriceFilter::decode_log(log) {
                return Ok(DarkForestGPTCreditEvents::ChangedCreditPriceFilter(decoded));
            }
            if let Ok(decoded) = DeductedCreditsFilter::decode_log(log) {
                return Ok(DarkForestGPTCreditEvents::DeductedCreditsFilter(decoded));
            }
            if let Ok(decoded) = GiftedCreditsFilter::decode_log(log) {
                return Ok(DarkForestGPTCreditEvents::GiftedCreditsFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
