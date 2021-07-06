use ark_ec::PairingEngine;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};

use super::r1cs_reader::R1CS;

#[derive(Clone)]
pub struct CircomCircuit<E: PairingEngine> {
    pub r1cs: R1CS<E>,
    pub witness: Option<Vec<E::Fr>>,
}

impl<'a, E: PairingEngine> CircomCircuit<E> {
    pub fn get_public_inputs(&self) -> Option<Vec<E::Fr>> {
        match &self.witness {
            None => None,
            Some(w) => match &self.r1cs.wire_mapping {
                None => Some(w[1..self.r1cs.num_inputs].to_vec()),
                Some(m) => Some(m[1..self.r1cs.num_inputs].iter().map(|i| w[*i]).collect()),
            },
        }
    }

    // pub fn get_public_inputs_json(&self) -> String {
    //     let inputs = self.get_public_inputs();
    //     let inputs = match inputs {
    //         None => return String::from("[]"),
    //         Some(inp) => inp.iter().map(|x| repr_to_big(x.into_repr())).collect_vec(),
    //     };
    //     serde_json::to_string_pretty(&inputs).unwrap()
    // }
}

impl<E: PairingEngine> ConstraintSynthesizer<E::Fr> for CircomCircuit<E> {
    fn generate_constraints(self, cs: ConstraintSystemRef<E::Fr>) -> Result<(), SynthesisError> {
        let witness = &self.witness;
        let wire_mapping = &self.r1cs.wire_mapping;

        for i in 1..self.r1cs.num_inputs {
            cs.new_input_variable(|| {
                Ok(match witness {
                    None => E::Fr::from(1u32),
                    Some(w) => match wire_mapping {
                        None => w[i],
                        Some(m) => w[m[i]],
                    },
                })
            })?;
        }

        for i in 1..self.r1cs.num_aux {
            cs.new_witness_variable(|| {
                Ok(match witness {
                    None => E::Fr::from(1u32),
                    Some(w) => match wire_mapping {
                        None => w[i + self.r1cs.num_inputs],
                        Some(m) => w[m[i + self.r1cs.num_inputs]],
                    },
                })
            })?;
        }

        let make_index = |index| {
            if index < self.r1cs.num_inputs {
                Variable::Instance(index)
            } else {
                Variable::Witness(index - self.r1cs.num_inputs)
            }
        };
        let make_lc = |lc_data: Vec<(usize, E::Fr)>| {
            lc_data.iter().fold(
                LinearCombination::<E::Fr>::zero(),
                |lc: LinearCombination<E::Fr>, (index, coeff)| lc + (*coeff, make_index(*index)),
            )
        };

        for constraint in &self.r1cs.constraints {
            cs.enforce_constraint(
                make_lc(constraint.0.clone()),
                make_lc(constraint.1.clone()),
                make_lc(constraint.2.clone()),
            )?;
        }
        Ok(())
    }
}
