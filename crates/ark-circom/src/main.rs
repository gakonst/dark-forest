use std::collections::HashMap;

// use ark_circom::WitnessCalculator;
use color_eyre::Result;

use ark_bn254::Bn254;

fn main() -> Result<()> {
    // let mut wtns_calc = WitnessCalculator::<Bn254>::new("/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit.wasm")?;

    // // 1. calculate witness for the inputs
    // let mut inputs = HashMap::new();
    // inputs.insert("a".to_string(), vec![ark_ff::BigInteger256::from(3)]);
    // inputs.insert("b".to_string(), vec![ark_ff::BigInteger256::from(11)]);

    // let res = wtns_calc.calculate_witness(inputs, true)?;

    // 2. create the circom circuit (zkutil)

    // 3. generate the params / proof

    // 4. make sure it verifies

    Ok(())
}
