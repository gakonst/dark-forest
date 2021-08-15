// TODO: Fix these warnings in ethers-rs
#![allow(clippy::type_complexity)]
#![allow(clippy::redundant_clone)]
// mod darkforesttypes;
// pub use darkforesttypes::*;
//
// mod darkforestgetters;
// pub use darkforestgetters::*;
//
// mod whitelist;
// pub use whitelist::*;

// mod darkforestcore;
// pub use darkforestcore::*;
//
// mod darkforeststoragev1;
// pub use darkforeststoragev1::*;
//
// mod verifier;
// pub use verifier::*;
//
// mod darkforestinitialize;
// pub use darkforestinitialize::*;
//
// mod darkforesttokens;
// pub use darkforesttokens::*;
//
// mod darkforestgptcredit;
// pub use darkforestgptcredit::*;

use ethers::contract::abigen;

macro_rules! multi_abigen {
       ($($contract:ident, $path:literal $(, methods
            { $($method_name:ident ($($arg:ty),*) as  $alias:ident;)* }  )? );* $(;)?) => {
        $(
              abigen!(
                  $contract,
                  $path
                  $(,
                      methods {
                              $(
                                $method_name($($arg),*) as $alias;
                              )*
                      }
                  )?
              );
        )*

    };
}

multi_abigen! {
    DarkForestCore, "./abis/DarkForestCore.json";
    DarkForestGetters, "./abis/DarkForestGetters.json";
    DarkForestGPTCredit, "./abis/DarkForestGPTCredit.json";
    DarkForestInitialize, "./abis/DarkForestInitialize.json";
    DarkForestStorageV1, "./abis/DarkForestStorageV1.json";
    DarkForestTokens, "./abis/DarkForestTokens.json",
        methods {
            safeTransferFrom(address,address,uint256,bytes) as safe_transfer_from_data;
        }
    ;
    Verifier, "./abis/Verifier.json";
    Whitelist, "./abis/Whitelist.json";
}
