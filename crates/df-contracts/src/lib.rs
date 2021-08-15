pub mod config;
pub use config::{Config, Network};

pub fn addr(s: &str) -> ethers::types::Address {
    s.strip_prefix("0x").unwrap_or(s).parse().unwrap()
}

pub use bindings::*;
pub mod bindings {
    use ethers::contract::abigen;

    macro_rules! multi_abigen {
       ($($contract:ident, $path:literal
            $(, methods { $($method_name:ident ($($arg:ty),*) as  $alias:ident;)* } )?
            $(, event_derives ($($derives:path),*))?
       );* $(;)?
       ) => {
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
                  $(,
                    event_derives ($($derives,)*),
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
}
