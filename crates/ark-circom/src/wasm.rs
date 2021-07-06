use std::cell::Cell;

use color_eyre::Result;
use wasmer::{imports, Function, Instance, Memory, MemoryType, Module, Store, Value};

use num_traits::Zero;

use num_bigint::BigInt;
use std::str::FromStr;

use crate::{fnv, memory::SafeMem};

#[derive(Clone, Debug)]
pub struct WitnessCalculator {
    pub instance: CircomInstance,
    pub memory: SafeMem,

    pub n32: i32,
    pub n64: i32,

    pub prime: BigInt,
    pub r_inv: BigInt,
}

impl WitnessCalculator {
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
        let instance = CircomInstance(Instance::new(&module, &import_object)?);

        let n32 = (instance.get_fr_len()? >> 2) - 2;

        let r_inv = BigInt::from_str(
            "9915499612839321149637521777990102151350674507940716049588462388200839649614",
        )
        .unwrap();
        let mut memory = SafeMem::new(memory, n32 as usize, BigInt::zero(), r_inv.clone());
        let ptr = instance.get_ptr_raw_prime()?;
        let prime = memory.read_big(ptr as usize, n32 as usize)?;
        let n64 = ((prime.bits() - 1) / 64 + 1) as i32;
        memory.prime = prime.clone();

        Ok(WitnessCalculator {
            instance,
            memory,
            n32,
            prime,
            r_inv,
            n64,
        })
    }

    pub fn calculate_witness<I: IntoIterator<Item = (String, Vec<BigInt>)>>(
        &mut self,
        inputs: I,
        sanity_check: bool,
    ) -> Result<Vec<BigInt>> {
        let old_mem_free_pos = self.memory.free_pos();

        self.instance.init(sanity_check)?;

        let p_sig_offset = self.memory.alloc_u32();
        let p_fr = self.memory.alloc_fr();

        // allocate the inputs
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = fnv(&name);
            self.instance
                .get_signal_offset32(p_sig_offset, 0, msb, lsb)?;

            let sig_offset = self.memory.read_u32(p_sig_offset as usize) as usize;

            for (i, value) in values.into_iter().enumerate() {
                self.memory.write_fr(p_fr as usize, &value)?;
                self.instance
                    .set_signal(0, 0, (sig_offset + i) as i32, p_fr as i32)?;
            }
        }

        let mut w = Vec::new();
        let n_vars = self.instance.get_n_vars()?;
        for i in 0..n_vars {
            let ptr = self.instance.get_ptr_witness(i)? as usize;
            let el = self.memory.read_fr(ptr)?;
            w.push(el);
        }

        self.memory.set_free_pos(old_mem_free_pos);

        Ok(w)
    }

    pub fn get_witness_buffer(&self) -> Result<Vec<u8>> {
        let ptr = self.instance.get_ptr_witness_buffer()? as usize;

        let view = self.memory.memory.view::<u8>();

        let len = self.instance.get_n_vars()? * self.n64 * 8;
        let arr = view[ptr..ptr + len as usize]
            .iter()
            .map(Cell::get)
            .collect::<Vec<_>>();

        Ok(arr)
    }
}

#[derive(Clone, Debug)]
pub struct CircomInstance(Instance);

// binds to the circom functions
impl CircomInstance {
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
        self.0
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
    use super::*;
    use std::{collections::HashMap, path::PathBuf};

    struct TestCase<'a> {
        circuit_path: &'a str,
        inputs_path: &'a str,
        n_vars: u32,
        n64: u32,
        witness: &'a [&'a str],
    }

    pub fn root_path(p: &str) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(p);
        path.to_string_lossy().to_string()
    }

    #[test]
    fn multiplier_1() {
        run_test(TestCase {
            circuit_path: root_path("test-vectors/mycircuit.wasm").as_str(),
            inputs_path: root_path("test-vectors/mycircuit-input1.json").as_str(),
            n_vars: 4,
            n64: 4,
            witness: &["1", "33", "3", "11"],
        });
    }

    #[test]
    fn multiplier_2() {
        run_test(TestCase {
            circuit_path: root_path("test-vectors/mycircuit.wasm").as_str(),
            inputs_path: root_path("test-vectors/mycircuit-input2.json").as_str(),
            n_vars: 4,
            n64: 4,
            witness: &[
                "1",
                "21888242871839275222246405745257275088548364400416034343698204186575672693159",
                "21888242871839275222246405745257275088548364400416034343698204186575796149939",
                "11",
            ],
        });
    }

    #[test]
    fn multiplier_3() {
        run_test(TestCase {
            circuit_path: root_path("test-vectors/mycircuit.wasm").as_str(),
            inputs_path: root_path("test-vectors/mycircuit-input3.json").as_str(),
            n_vars: 4,
            n64: 4,
            witness: &[
                "1",
                "21888242871839275222246405745257275088548364400416034343698204186575808493616",
                "10944121435919637611123202872628637544274182200208017171849102093287904246808",
                "2",
            ],
        });
    }

    #[test]
    fn safe_multipler() {
        let witness =
            std::fs::read_to_string(&root_path("test-vectors/safe-circuit-witness.json")).unwrap();
        let witness: Vec<String> = serde_json::from_str(&witness).unwrap();
        let witness = &witness.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
        run_test(TestCase {
            circuit_path: root_path("test-vectors/circuit2.wasm").as_str(),
            inputs_path: root_path("test-vectors/mycircuit-input1.json").as_str(),
            n_vars: 132, // 128 + 4
            n64: 4,
            witness,
        });
    }

    #[test]
    fn smt_verifier() {
        let witness =
            std::fs::read_to_string(&root_path("test-vectors/smtverifier10-witness.json")).unwrap();
        let witness: Vec<String> = serde_json::from_str(&witness).unwrap();
        let witness = &witness.iter().map(|x| x.as_ref()).collect::<Vec<_>>();

        run_test(TestCase {
            circuit_path: root_path("test-vectors/smtverifier10.wasm").as_str(),
            inputs_path: root_path("test-vectors/smtverifier10-input.json").as_str(),
            n_vars: 4794,
            n64: 4,
            witness,
        });
    }

    use serde_json::Value;
    use std::str::FromStr;

    fn value_to_bigint(v: Value) -> BigInt {
        match v {
            Value::String(inner) => BigInt::from_str(&inner).unwrap(),
            Value::Number(inner) => BigInt::from(inner.as_u64().expect("not a u32")),
            _ => panic!("unsupported type"),
        }
    }

    fn run_test(case: TestCase) {
        let mut wtns = WitnessCalculator::new(case.circuit_path).unwrap();
        assert_eq!(
            wtns.prime.to_str_radix(16),
            "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001".to_lowercase()
        );
        assert_eq!(wtns.instance.get_n_vars().unwrap() as u32, case.n_vars);
        assert_eq!(wtns.n64 as u32, case.n64);

        let inputs_str = std::fs::read_to_string(case.inputs_path).unwrap();
        let inputs: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(&inputs_str).unwrap();

        let inputs = inputs
            .iter()
            .map(|(key, value)| {
                let res = match value {
                    Value::String(inner) => {
                        vec![BigInt::from_str(inner).unwrap()]
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

        let res = wtns.calculate_witness(inputs, false).unwrap();
        for i in 0..res.len() {
            assert_eq!(res[i], BigInt::from_str(case.witness[i]).unwrap());
        }
    }
}
