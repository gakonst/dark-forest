use std::collections::HashMap;

use ark_circom::{
    circuit::{circom::CircomCircuit, r1cs_reader::R1CSFile},
    WitnessCalculator,
};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_std::rand::thread_rng;
use color_eyre::Result;
use num_bigint::{BigInt, ToBigUint};

use ark_bn254::{Bn254, Fr};

use ark_groth16::generate_random_parameters;

fn main() -> Result<()> {
    let mut wtns_calc = WitnessCalculator::new("./test-vectors/mycircuit.wasm")?;

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
    dbg!(&res);

    let wtns = res
        .into_iter()
        .map(|w| Fr::from(w.to_biguint().unwrap()))
        .collect::<Vec<_>>();

    let r1cs = std::fs::File::open("./test-vectors/mycircuit.r1cs")?;
    let r1cs = R1CSFile::<Bn254>::new(r1cs)?.into();

    let circom: CircomCircuit<Bn254> = CircomCircuit {
        r1cs,
        witness: Some(wtns),
    };
    dbg!(&circom);

    use ark_relations::r1cs::ConstraintSystem;
    let cs = ConstraintSystem::<Fr>::new_ref();

    circom.generate_constraints(cs.clone())?;

    dbg!(cs.is_satisfied());


    // let mut rng = thread_rng();
    // let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    // dbg!(&params);

    // 2. create the circom circuit (zkutil)

    // 3. generate the params / proof

    // 4. make sure it verifies

    Ok(())
}
