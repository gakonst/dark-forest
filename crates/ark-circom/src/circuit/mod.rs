use ark_ec::PairingEngine;

pub mod r1cs_reader;

pub type ConstraintSet<E> = (
    Vec<(usize, <E as PairingEngine>::Fr)>,
    Vec<(usize, <E as PairingEngine>::Fr)>,
    Vec<(usize, <E as PairingEngine>::Fr)>,
);

pub type ConstraintVec<E> = Vec<(usize, <E as PairingEngine>::Fr)>;
