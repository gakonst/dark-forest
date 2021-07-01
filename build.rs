#![allow(unused)]
use ethers::contract::Abigen;

// const OUR_CONTRACTS: &[&'static str] = &[

const DF_CONTRACTS: &[&str] = &[
    "DarkForestCore",
    "DarkForestGPTCredit",
    "DarkForestGetters",
    "DarkForestInitialize",
    "DarkForestStorageV1",
    "DarkForestTokens",
    "DarkForestTypes",
    "Verifier",
    "Whitelist",
    // Libs
    // "DarkForestUtils",
    // "DarkForestLazyUpdate",
    // "DarkForestPlanet",
    // "DarkForestArtifactUtils",
];

fn main() {
    // for name in OUR_CONTRACTS {
    //     println!("cargo:rerun-if-changed=./contracts/{}.abi", name);
    //     bindgen(name);
    // }

    // for name in DF_CONTRACTS {
    //     println!("cargo:rerun-if-changed=./abis/{}.abi", name);
    //     bindgen(name);
    // }
}

#[allow(dead_code)]
fn bindgen(fname: &'static str) {
    let abigen =
        Abigen::new(fname, format!("./abis/{}.json", fname)).expect("could not instantiate Abigen");

    // special cases
    let abigen = match fname {
        "DarkForestTokens" => abigen.add_method_alias(
            "safeTransferFrom(address,address,uint256,bytes)",
            "safe_transfer_from_data",
        ),
        _ => abigen,
    };

    let bindings = abigen
        .generate()
        .unwrap_or_else(|_| panic!("could not generate bindings for {}", fname));

    bindings
        .write_to_file(format!("./src/bindings/{}.rs", fname.to_lowercase()))
        .expect("could not write bindings to file");
}
