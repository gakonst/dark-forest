mod types;
pub use types::{Inputs, Proof, VerifyingKey, G1, G2};

use ethers::{
    contract::{abigen, ContractError},
    providers::Middleware,
    types::Address,
};

use std::sync::Arc;

// TODO, make this something else? Make it test only? Read it from an ABI file?
abigen!(
    Verifier,
    r#"[{"inputs":[{"internalType":"uint256[]","name":"input","type":"uint256[]"},{"components":[{"components":[{"internalType":"uint256","name":"X","type":"uint256"},{"internalType":"uint256","name":"Y","type":"uint256"}],"internalType":"struct Pairing.G1Point","name":"A","type":"tuple"},{"components":[{"internalType":"uint256[2]","name":"X","type":"uint256[2]"},{"internalType":"uint256[2]","name":"Y","type":"uint256[2]"}],"internalType":"struct Pairing.G2Point","name":"B","type":"tuple"},{"components":[{"internalType":"uint256","name":"X","type":"uint256"},{"internalType":"uint256","name":"Y","type":"uint256"}],"internalType":"struct Pairing.G1Point","name":"C","type":"tuple"}],"internalType":"struct Verifier.Proof","name":"proof","type":"tuple"},{"components":[{"components":[{"internalType":"uint256","name":"X","type":"uint256"},{"internalType":"uint256","name":"Y","type":"uint256"}],"internalType":"struct Pairing.G1Point","name":"alfa1","type":"tuple"},{"components":[{"internalType":"uint256[2]","name":"X","type":"uint256[2]"},{"internalType":"uint256[2]","name":"Y","type":"uint256[2]"}],"internalType":"struct Pairing.G2Point","name":"beta2","type":"tuple"},{"components":[{"internalType":"uint256[2]","name":"X","type":"uint256[2]"},{"internalType":"uint256[2]","name":"Y","type":"uint256[2]"}],"internalType":"struct Pairing.G2Point","name":"gamma2","type":"tuple"},{"components":[{"internalType":"uint256[2]","name":"X","type":"uint256[2]"},{"internalType":"uint256[2]","name":"Y","type":"uint256[2]"}],"internalType":"struct Pairing.G2Point","name":"delta2","type":"tuple"},{"components":[{"internalType":"uint256","name":"X","type":"uint256"},{"internalType":"uint256","name":"Y","type":"uint256"}],"internalType":"struct Pairing.G1Point[]","name":"IC","type":"tuple[]"}],"internalType":"struct Verifier.VerifyingKey","name":"vk","type":"tuple"}],"name":"verify","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"}]"#,
);

pub struct Groth16Verifier<M> {
    verifier: Verifier<M>,
}

impl<M: Middleware> Groth16Verifier<M> {
    pub fn new(provider: Arc<M>, address: Address) -> Self {
        Self {
            verifier: Verifier::new(address, provider),
        }
    }

    pub async fn check_proof<I: Into<Inputs>, P: Into<Proof>, VK: Into<VerifyingKey>>(
        &self,
        vk: VK,
        proof: P,
        inputs: I,
    ) -> Result<bool, ContractError<M>> {
        let proof = proof.into().as_tuple();
        let vk = vk.into().as_tuple();
        let inputs = inputs.into().0;

        let ok = self.verifier.verify(inputs, proof, vk).call().await?;
        Ok(ok)
    }
}