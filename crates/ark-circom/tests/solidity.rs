use ark_circom::{solidity_compat::Groth16Verifier, CircomBuilder, CircuitConfig};
use ark_std::rand::thread_rng;
use color_eyre::Result;

use ark_bn254::Bn254;
use ark_groth16::{create_random_proof as prove, generate_random_parameters};

use ethers::{
    contract::ContractFactory,
    providers::{Http, Provider},
    utils::{compile_and_launch_ganache, Ganache, Solc},
};

use std::{convert::TryFrom, sync::Arc};

#[tokio::test]
async fn solidity_verifier() -> Result<()> {
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
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    let circom = builder.build()?;
    let inputs = circom.get_public_inputs().unwrap();

    let proof = prove(circom, &params, &mut rng)?;

    // launch the network
    let (compiled, ganache) = compile_and_launch_ganache(
        Solc::new("./src/solidity_compat/verifier.sol"),
        Ganache::new(),
    )
    .await?;

    let acc = ganache.addresses()[0];

    let contract = compiled
        .get("TestVerifier")
        .expect("could not find contract");

    let provider = Provider::<Http>::try_from(ganache.endpoint())?;
    let provider = provider.with_sender(acc);
    let provider = Arc::new(provider);

    let factory = ContractFactory::new(
        contract.abi.clone(),
        contract.bytecode.clone(),
        provider.clone(),
    );

    let contract = factory.deploy(())?.send().await?;
    let addr = contract.address();

    let contract = Groth16Verifier::new(provider, addr);

    let verified = contract
        .check_proof(params.vk, proof, inputs.as_slice())
        .await?;
    assert!(verified);

    Ok(())
}
