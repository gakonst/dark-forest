use std::cell::Cell;
use std::marker::PhantomData;

use color_eyre::Result;
use wasmer::{imports, Function, Instance, Memory, MemoryType, MemoryView, Module, Store, Value};

use num_traits::{One, ToPrimitive, Zero};

use ark_ec::PairingEngine;
use ark_ff::{BigInteger, BigInteger256 as BigInt, PrimeField};
use ark_ff::FromBytes;

use crate::{fnv, memory::SafeMem};

#[derive(Clone, Debug)]
pub struct WitnessCalculator<E: PairingEngine> {
    pub instance: Instance,
    pub memory: SafeMem,

    pub n32: i32,
    pub n64: i32,

    pub prime: BigInt,
    pub r_inv: BigInt,

    engine: PhantomData<E>,
}

impl<E: PairingEngine> WitnessCalculator<E> {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let store = Store::default();
        let module = Module::from_file(&store, path)?;

        // Set up the memory
        let memory = Memory::new(&store, MemoryType::new(2000, None, false)).unwrap();
        let import_object = imports! {
            "env" => {
                "memory" => memory.clone(),
            },
            // Host function callbacks from the WASM
            "runtime" => {
                "error" => runtime::error(&store),
                "logSetSignal" => runtime::log_signal(&store),
                "logGetSignal" => runtime::log_signal(&store),
                "logFinishComponent" => runtime::log_component(&store),
                "logStartComponent" => runtime::log_component(&store),
                "log" => runtime::log_component(&store),
            }
        };
        let instance = Instance::new(&module, &import_object)?;

        let mut wns = WitnessCalculator {
            instance,
            memory: SafeMem::new(memory),
            n32: 0,
            engine: PhantomData,
            prime: BigInt::from(0),
            r_inv: BigInt::read(
                &hex::decode(
                    "9915499612839321149637521777990102151350674507940716049588462388200839649614",
                )
                .unwrap()[..],
            )
            .unwrap(),
            n64: 0,
        };

        wns.n32 = (wns.get_fr_len()? >> 2) - 2;

        // load n32 bytes from the start of the raw Fr ptr
        let ptr = wns.get_ptr_raw_prime()?;
        let prime = wns.memory.read_big(ptr as usize, wns.n32 as usize)?;

        let nvars = wns.get_n_vars()?;
        let n64 = (prime.num_bits() - 1) / 64 + 1;
        wns.n64 = n64 as i32;

        // let mut r = <E::Fr as PrimeField>::BigInt::from(1);
        // r.muln(n64 * 64);
        // let r = E::Fr::from_repr(r).unwrap();

        wns.n32 = wns.get_fr_len()? - 8;
        wns.prime = prime;

        Ok(wns)
    }

    pub fn calculate_witness<I: IntoIterator<Item = (String, Vec<BigInt>)>>(
        &mut self,
        inputs: I,
        sanity_check: bool,
    ) -> Result<Vec<BigInt>> {
        let old_mem_free_pos = self.memory.free_pos();

        self.init(sanity_check)?;

        let p_sig_offset = self.memory.alloc_u32();
        let p_fr = self.memory.alloc_fr();

        // allocate the inputs
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = fnv(&name);
            self.get_signal_offset32(p_sig_offset, 0, msb, lsb)?;
            println!("hash({}) = ({}, {})", name, msb, lsb);

            let sig_offset = self.memory.read_u32(p_sig_offset as usize) as usize;

            for (i, value) in values.into_iter().enumerate() {
                self.memory.write_fr(p_fr as usize, value)?;
                self.set_signal(0, 0, (sig_offset + i) as i32, p_fr as i32)?;
            }
        }

        let mut w = Vec::new();
        let n_vars = self.get_n_vars()?;
        for i in 0..n_vars {
            let ptr = self.get_ptr_witness(i)? as usize;

            let el = self.load_fr(ptr)?;
            w.push(el);
            println!("ptr({}) => {}", ptr, el);
        }

        self.memory.set_free_pos(old_mem_free_pos);

        Ok(w)
    }

    // https://github.com/iden3/go-circom-witnesscalc/blob/25592ab9b33bf8d6b99c133783bd208bee7a935c/witnesscalc.go#L410-L430
    pub fn load_fr(&self, ptr: usize) -> Result<BigInt> {
        let view = self.memory.memory.view::<u8>();
        let ty = view[ptr + 4 + 3].get();

        let res = if ty & 0x80 != 0 {
            dbg!("not 0x80");
            let res = self.memory.read_big(ptr + 8, self.n32 as usize)?;
            if ty & 0x40 != 0 {
                self.from_montgomery(res)
            } else {
                res
            }
        } else if view[ptr + 3].get() & 0x40 != 0 {
            let res = self.memory.read_big(ptr, 4)?;
            res
        } else {
            self.memory.read_big(ptr, 1)?
        };

        Ok(res)
    }

    // TODO: This is 99% wrong
    fn from_montgomery(&self, res: BigInt) -> BigInt {
        use std::ops::Mul;
        let r_inv = ark_bn254::Fr::from_repr(self.r_inv).unwrap();
        ark_bn254::Fr::from_repr(res)
            .unwrap()
            .mul(r_inv)
            .into_repr()
    }

    pub fn get_witness_buffer(&self) -> Result<Vec<u8>> {
        let ptr = self.get_i32("getWitnessBuffer")? as usize;

        let view = self.memory.memory.view::<u8>();

        let len = self.get_n_vars()? * self.n64 * 8;
        let arr = view[ptr..ptr + len as usize]
            .iter()
            .map(Cell::get)
            .collect::<Vec<_>>();

        Ok(arr)
    }
}

// binds to the circom functions
impl<E: PairingEngine> WitnessCalculator<E> {
    pub fn init(&self, sanity_check: bool) -> Result<()> {
        let func = self.func("init");
        func.call(&[Value::I32(sanity_check as i32)])?;
        Ok(())
    }

    pub fn get_fr_len(&self) -> Result<i32> {
        self.get_i32("getFrLen")
    }

    pub fn get_ptr_raw_prime(&self) -> Result<i32> {
        self.get_i32("getPRawPrime")
    }

    pub fn get_n_vars(&self) -> Result<i32> {
        self.get_i32("getNVars")
    }

    pub fn get_ptr_witness_buffer(&self) -> Result<i32> {
        self.get_i32("getWitnessBuffer")
    }

    pub fn get_ptr_witness(&self, w: i32) -> Result<i32> {
        let func = self.func("getPWitness");
        let res = func.call(&[w.into()])?;

        Ok(res[0].unwrap_i32())
    }

    pub fn get_signal_offset32(
        &self,
        p_sig_offset: u32,
        component: u32,
        hash_msb: u32,
        hash_lsb: u32,
    ) -> Result<()> {
        let func = self.func("getSignalOffset32");
        func.call(&[
            p_sig_offset.into(),
            component.into(),
            hash_msb.into(),
            hash_lsb.into(),
        ])?;

        Ok(())
    }

    pub fn set_signal(&self, c_idx: i32, component: i32, signal: i32, p_val: i32) -> Result<()> {
        let func = self.func("setSignal");
        func.call(&[c_idx.into(), component.into(), signal.into(), p_val.into()])?;

        Ok(())
    }

    fn get_i32(&self, name: &str) -> Result<i32> {
        let func = self.func(name);
        let result = func.call(&[])?;
        Ok(result[0].unwrap_i32())
    }

    fn func(&self, name: &str) -> &Function {
        self.instance
            .exports
            .get_function(name)
            .expect(&format!("function {} not found", name))
    }
}

// callback hooks for debugging
mod runtime {
    use super::*;

    pub fn error(store: &Store) -> Function {
        #[allow(unused)]
        fn func(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) {}
        Function::new_native(&store, func)
    }

    pub fn log_signal(store: &Store) -> Function {
        #[allow(unused)]
        fn func(a: i32, b: i32) {}
        Function::new_native(&store, func)
    }

    pub fn log_component(store: &Store) -> Function {
        #[allow(unused)]
        fn func(a: i32) {}
        Function::new_native(&store, func)
    }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Bn254;
    use std::{collections::HashMap, path::PathBuf};

    use super::*;

    struct TestCase<'a> {
        circuit_path: &'a str,
        inputs_path: &'a str,
        n_vars: u32,
        witness: &'a [&'a str],
    }

    #[test]
    fn multiplier_1() {
        run_test(TestCase {
            circuit_path: "/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit.wasm",
            // inputs_path: "/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/smtverifier10-input.json",
            inputs_path: "/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit-input1.json",
            n_vars: 4,
            witness: &["1", "33", "3", "11"],
        });
    }

    #[test]
    fn multiplier_2() {
        run_test(TestCase {
            circuit_path: "/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit.wasm",
            inputs_path: "/Users/Georgios/paradigm/portfolio/dforest/dark-forest/crates/ark-circom/test-vectors/mycircuit-input2.json",
            n_vars: 4,
            witness: &["1","21888242871839275222246405745257275088548364400416034343698204186575672693159","21888242871839275222246405745257275088548364400416034343698204186575796149939","11"],
        });
    }

    use serde_json::Value;
    use std::str::FromStr;

    fn value_to_bigint(v: Value) -> BigInt {
        match v {
            Value::String(inner) => {
                dbg!(&inner);
                BigInt::from(u64::from_str(&inner).unwrap())
            }
            Value::Number(inner) => BigInt::from(inner.as_u64().expect("not a u32")),
            _ => panic!("unsupported type"),
        }
    }

    fn run_test(case: TestCase) {
        let mut wtns = WitnessCalculator::<Bn254>::new(case.circuit_path).unwrap();
        let inputs_str = std::fs::read_to_string(case.inputs_path).unwrap();
        let inputs: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(&inputs_str).unwrap();

        let inputs = inputs
            .iter()
            .map(|(key, value)| {
                let res = match value {
                    Value::String(inner) => {
                        vec![BigInt::from(u64::from_str(&inner).unwrap())]
                    }
                    Value::Number(inner) => {
                        vec![BigInt::from(inner.as_u64().expect("not a u32"))]
                    }
                    Value::Array(inner) => {
                        inner.into_iter().cloned().map(value_to_bigint).collect()
                    }
                    _ => panic!(),
                };

                (key.clone(), res)
            })
            .collect::<HashMap<_, _>>();

        let res = wtns.calculate_witness(inputs, true).unwrap();
        for i in 0..res.len() {
            // get the bottom 32 of the witness (how do we handle >u32 witnesses?)
            assert_eq!(res[i].0[0] as u32, case.witness[i].parse::<u32>().unwrap());
        }

        assert_eq!(
            wtns.prime.to_string(),
            "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001"
        );
        assert_eq!(wtns.get_n_vars().unwrap() as u32, case.n_vars);
    }
}
