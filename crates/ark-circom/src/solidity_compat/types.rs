use ark_ff::{BigInteger, PrimeField};
use ethers::types::U256;

use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};

pub struct Inputs(pub Vec<U256>);

impl From<&[Fr]> for Inputs {
    fn from(src: &[Fr]) -> Self {
        let els = src
            .iter()
            .map(|point| {
                let point = point.into_repr();
                let point_bytes = point.to_bytes_le();
                U256::from(&point_bytes[..])
            })
            .collect();

        Self(els)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct G1 {
    pub x: U256,
    pub y: U256,
}

type G1Tup = (U256, U256);

impl G1 {
    fn as_tuple(&self) -> (U256, U256) {
        (self.x, self.y)
    }
}

impl From<&G1Affine> for G1 {
    fn from(p: &G1Affine) -> Self {
        Self {
            x: point_to_u256(p.x),
            y: point_to_u256(p.y),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct G2 {
    pub x: [U256; 2],
    pub y: [U256; 2],
}

type G2Tup = ([U256; 2], [U256; 2]);

impl G2 {
    fn as_tuple(&self) -> G2Tup {
        ([self.x[0], self.x[1]], [self.y[0], self.y[1]])
    }
}

impl From<&G2Affine> for G2 {
    fn from(p: &G2Affine) -> Self {
        Self {
            x: [point_to_u256(p.x.c0), point_to_u256(p.x.c1)],
            y: [point_to_u256(p.y.c0), point_to_u256(p.y.c1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Proof {
    a: G1,
    b: G2,
    c: G1,
}

impl Proof {
    pub fn as_tuple(&self) -> (G1Tup, G2Tup, G1Tup) {
        (self.a.as_tuple(), self.b.as_tuple(), self.c.as_tuple())
    }
}

impl From<ark_groth16::Proof<Bn254>> for Proof {
    fn from(proof: ark_groth16::Proof<Bn254>) -> Self {
        Self {
            a: G1::from(&proof.a),
            b: G2::from(&proof.b),
            c: G1::from(&proof.c),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerifyingKey {
    alpha1: G1,
    beta2: G2,
    gamma2: G2,
    delta2: G2,
    ic: Vec<G1>,
}

impl VerifyingKey {
    pub fn as_tuple(&self) -> (G1Tup, G2Tup, G2Tup, G2Tup, Vec<G1Tup>) {
        (
            self.alpha1.as_tuple(),
            self.beta2.as_tuple(),
            self.gamma2.as_tuple(),
            self.delta2.as_tuple(),
            self.ic.iter().map(|i| i.as_tuple()).collect(),
        )
    }
}

impl From<ark_groth16::VerifyingKey<Bn254>> for VerifyingKey {
    fn from(vk: ark_groth16::VerifyingKey<Bn254>) -> Self {
        Self {
            alpha1: G1::from(&vk.alpha_g1),
            beta2: G2::from(&vk.beta_g2),
            gamma2: G2::from(&vk.gamma_g2),
            delta2: G2::from(&vk.delta_g2),
            ic: vk.gamma_abc_g1.iter().map(G1::from).collect(),
        }
    }
}

// Helper for converting a PrimeField to its U256 representation for Ethereum compatibility
fn point_to_u256<F: PrimeField>(point: F) -> U256 {
    let point = point.into_repr();
    let point_bytes = point.to_bytes_le();
    U256::from(&point_bytes[..])
}
