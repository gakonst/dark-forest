pub use darkforestinitialize_mod::*;
#[allow(clippy::too_many_arguments)]
mod darkforestinitialize_mod {
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
    #[doc = "DarkForestInitialize was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DARKFORESTINITIALIZE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| serde_json::from_str("[]\n").expect("invalid abi"));
    #[derive(Clone)]
    pub struct DarkForestInitialize<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DarkForestInitialize<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DarkForestInitialize<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DarkForestInitialize))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DarkForestInitialize<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                DARKFORESTINITIALIZE_ABI.clone(),
                client,
            );
            Self(contract)
        }
    }
}
