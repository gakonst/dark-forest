use ark_circom::{circom_qap::R1CStoQAPCircom, CircomBuilder, CircuitConfig};
use ark_std::rand::thread_rng;
use color_eyre::Result;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, create_random_proof_with_qap, generate_random_parameters,
    generate_random_parameters_with_qap, prepare_verifying_key, verify_proof,
};

#[test]
fn groth16_proof() -> Result<()> {
    let cfg = CircuitConfig::<Bn254>::new(
        "./test-vectors/mycircuit.wasm",
        "./test-vectors/mycircuit.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);

    // create an empty instance for setting it up
    let circom = builder.setup();

    let mut rng = thread_rng();

    let params = generate_random_parameters::<Bn254, _, _>(circom.clone(), &mut rng)?;
    let params_circom =
        generate_random_parameters_with_qap::<Bn254, _, _, R1CStoQAPCircom>(circom, &mut rng)?;

    let circom = builder.build()?;

    let inputs = circom.get_public_inputs().unwrap();

    let proof = prove(circom.clone(), &params, &mut rng)?;
    let proof_circom = create_random_proof_with_qap::<_, _, _, R1CStoQAPCircom>(
        circom.clone(),
        &params_circom,
        &mut rng,
    )?;

    let pvk = prepare_verifying_key(&params.vk);
    let verified = verify_proof(&pvk, &proof, &inputs)?;
    assert!(verified);

    let pvk_circom = prepare_verifying_key(&params_circom.vk);
    let verified = verify_proof(&pvk_circom, &proof_circom, &inputs)?;
    assert!(verified);

    Ok(())
}
