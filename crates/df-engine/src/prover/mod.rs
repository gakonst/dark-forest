use ark_bn254::Bn254;
use ark_circom::{CircomBuilder, CircomCircuit, CircomConfig, CircomReduction};
use ark_groth16::{create_random_proof_with_reduction, Proof, ProvingKey};

use ark_std::rand::thread_rng;
use df_types::planet::PlanetLocation;

// TODO: Add Reveal Prover & Find Artifact Prover
pub(crate) mod init;
use init::InitProver;

pub(crate) mod mover;
use mover::MoveProver;

/// A proof along with its corresponding public inputs
pub type ProofWithInputs = (Proof<Bn254>, Vec<ark_bn254::Fr>);

/// A prover generates Groth16 proofs over Bn254 for the Dark Forest
/// circuits
pub struct Prover {
    mover: MoveProver,
    initializer: InitProver,
}

impl Prover {
    /// Generates a Move proof
    pub fn prove_move(
        &self,
        from: &PlanetLocation,
        to: &PlanetLocation,
    ) -> Result<ProofWithInputs> {
        self.mover.prove(from, to)
    }

    /// Generates an InitializePlayer proof
    pub fn prove_initialize(&self, loc: &PlanetLocation) -> Result<ProofWithInputs> {
        self.initializer.prove(loc)
    }
}

/// Helper for proving things about circuits
pub struct CircuitProver {
    builder: CircomBuilder<Bn254>,
    pub params: ProvingKey<Bn254>,
}

use std::{fs::File, path::PathBuf};

use color_eyre::Result;

impl CircuitProver {
    pub fn new_path<P: Into<PathBuf>>(zkey: P, wasm: P, r1cs: P) -> Result<Self> {
        let cfg = CircomConfig::<Bn254>::new(wasm.into(), r1cs.into())?;
        let builder = CircomBuilder::new(cfg);

        let mut reader = File::open(zkey.into())?;
        let params = ark_circom::read_zkey(&mut reader)?;

        Ok(CircuitProver::new(builder, params))
    }

    pub fn new(builder: CircomBuilder<Bn254>, params: ProvingKey<Bn254>) -> Self {
        Self { builder, params }
    }
}

fn prove(circuit: CircomCircuit<Bn254>, params: &ProvingKey<Bn254>) -> Result<ProofWithInputs> {
    // TODO: Make this a Result
    let public_inputs = circuit.get_public_inputs().unwrap();
    let proof = create_random_proof_with_reduction::<_, _, _, CircomReduction>(
        circuit,
        params,
        &mut thread_rng(),
    )?;
    Ok((proof, public_inputs))
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use ark_circom::CircomConfig;
    use ark_groth16::{
        generate_random_parameters_with_reduction, prepare_verifying_key, verify_proof,
    };
    use std::fs::File;

    use crate::tests::{from_planet, root_path, to_planet};

    pub fn circuit_prover<T: From<CircuitProver>>(zkey: Option<&str>, circuit_type: &str) -> T {
        let cfg = CircomConfig::<Bn254>::new(
            // todo: make this path configurable too?
            root_path(&format!("./round3-data/{}.wasm", circuit_type)),
            root_path(&format!("./round3-data/{}.r1cs", circuit_type)),
        )
        .unwrap();
        let builder = CircomBuilder::new(cfg);

        let circom = builder.setup();
        let mut rng = thread_rng();

        let params = if let Some(zkey) = zkey {
            let mut reader = File::open(root_path(zkey)).unwrap();
            ark_circom::read_zkey(&mut reader).unwrap()
        } else {
            generate_random_parameters_with_reduction::<Bn254, _, _, CircomReduction>(
                circom, &mut rng,
            )
            .unwrap()
        };
        T::from(CircuitProver::new(builder, params))
    }

    fn check_prover(zkey: Option<&str>) {
        let prover: MoveProver = circuit_prover(zkey, "move");
        let (proof, inputs) = prover.prove(&from_planet(), &to_planet()).unwrap();
        let pvk = prepare_verifying_key(&prover.0.params.vk);
        let verified = verify_proof(&pvk, &proof, &inputs).unwrap();
        assert!(verified);
    }

    fn check_initializer(zkey: Option<&str>) {
        let prover: InitProver = circuit_prover(zkey, "init");
        let (proof, inputs) = prover.prove(&to_planet()).unwrap();
        let pvk = prepare_verifying_key(&prover.0.params.vk);
        let verified = verify_proof(&pvk, &proof, &inputs).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_prover() {
        check_prover(None);
    }

    #[test]
    fn test_prover_v06_round3() {
        check_prover(Some("./round3-data/move.zkey"));
    }

    #[test]
    fn test_initializer() {
        check_initializer(None);
    }

    #[test]
    fn test_initializer_v06_round3() {
        check_initializer(Some("./round3-data/init.zkey"));
    }
}
