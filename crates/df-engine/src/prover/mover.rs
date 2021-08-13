use super::{prove, CircuitProver, ProofWithInputs};
use ark_bn254::Bn254;
use ark_circom::CircomCircuit;
use df_types::{constants, planet::PlanetLocation};

pub struct MoveProver(pub CircuitProver);

impl From<CircuitProver> for MoveProver {
    fn from(src: CircuitProver) -> Self {
        Self(src)
    }
}

use color_eyre::Result;

impl MoveProver {
    pub fn args(&self, from: &PlanetLocation, to: &PlanetLocation) -> Result<CircomCircuit<Bn254>> {
        let mut builder = self.0.builder.clone();
        builder.push_input("x1", from.coords.x);
        builder.push_input("y1", from.coords.y);
        builder.push_input("x2", to.coords.x);
        builder.push_input("y2", to.coords.y);
        builder.push_input("r", to.max_radius());
        builder.push_input("distMax", to.max_distance(from));
        builder.push_input("PLANETHASH_KEY", constants::PLANETHASH_KEY);
        builder.push_input("SPACETYPE_KEY", constants::SPACETYPE_KEY);
        builder.push_input("SCALE", constants::PERLIN_LENGTH_SCALE);
        builder.push_input("xMirror", constants::PERLIN_MIRROR_X as u8);
        builder.push_input("yMirror", constants::PERLIN_MIRROR_Y as u8);
        builder.build()
    }

    pub fn prove(&self, from: &PlanetLocation, to: &PlanetLocation) -> Result<ProofWithInputs> {
        let circuit = self.args(from, to)?;
        prove(circuit, &self.0.params)
    }
}
