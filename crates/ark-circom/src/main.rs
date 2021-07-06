use std::collections::HashMap;

use ark_circom::{BigInt, WitnessCalculator};
use color_eyre::Result;

fn main() -> Result<()> {
    let mut wtns_calc = WitnessCalculator::new("/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit.wasm")?;

    // 1. calculate witness for the inputs
    let mut inputs = HashMap::new();
    inputs.insert(
        "a".to_string(),
        vec![
            "21888242871839275222246405745257275088548364400416034343698204186575796149939"
                .parse()
                .unwrap(),
        ],
    );
    inputs.insert("b".to_string(), vec![BigInt::from(11)]);

    let res = wtns_calc.calculate_witness(inputs, true)?;
    dbg!(res);

    // 2. create the circom circuit (zkutil)

    // 3. generate the params / proof

    // 4. make sure it verifies

    Ok(())
}
