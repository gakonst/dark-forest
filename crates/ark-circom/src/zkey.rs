//! ZKey
//!
//! Each ZKey file is broken into sections:
//!  Header(1)
//!       Prover Type 1 Groth
//!  HeaderGroth(2)
//!       n8q
//!       q
//!       n8r
//!       r
//!       NVars
//!       NPub
//!       DomainSize  (multiple of 2
//!       alpha1
//!       beta1
//!       delta1
//!       beta2
//!       gamma2
//!       delta2
//!  IC(3)
//!  Coefs(4)
//!  PointsA(5)
//!  PointsB1(6)
//!  PointsB2(7)
//!  PointsC(8)
//!  PointsH(9)
//!  Contributions(10)
use ark_ff::{BigInteger256, FromBytes};
use ark_serialize::{CanonicalDeserialize, SerializationError};
use ark_std::log2;
use byteorder::{LittleEndian, ReadBytesExt};

use std::{
    collections::HashMap,
    io::{Cursor, Read, Result as IoResult},
};

use ark_bn254::{Bn254, Fq, Fq2, G1Affine, G2Affine};
use ark_groth16::{ProvingKey, VerifyingKey};
use ark_serialize::CanonicalSerialize;
use num_traits::{Zero, One};

#[derive(Clone, Debug)]
pub struct Section {
    position: u64,
    size: usize,
}

#[derive(Debug)]
pub struct BinFile<'a> {
    ftype: String,
    version: u32,
    sections: HashMap<u32, Vec<Section>>,
    reader: &'a mut Cursor<&'a [u8]>,
}

impl<'a> BinFile<'a> {
    pub fn new(reader: &'a mut Cursor<&'a [u8]>) -> IoResult<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        let version = reader.read_u32::<LittleEndian>()?;

        let num_sections = reader.read_u32::<LittleEndian>()?;

        let mut sections = HashMap::new();
        for _ in 0..num_sections {
            let section_id = reader.read_u32::<LittleEndian>()?;
            let section_length = reader.read_u64::<LittleEndian>()?;

            let section = sections.entry(section_id).or_insert(Vec::new());
            section.push(Section {
                position: reader.position(),
                size: section_length as usize,
            });

            reader.set_position(reader.position() + section_length);
        }

        Ok(Self {
            ftype: std::str::from_utf8(&magic[..]).unwrap().to_string(),
            version,
            sections,
            reader,
        })
    }

    pub fn proving_key(&mut self) -> IoResult<ProvingKey<Bn254>> {
        let header = self.groth_header()?;
        dbg!(&header);
        let ic = self.ic(header.n_public)?;

        let a_query = self.a_query(header.n_vars)?;
        let b_g1_query = self.b_g1_query(header.n_vars)?;
        let b_g2_query = self.b_g2_query(header.n_vars)?;
        // TODO how many?
        let l_query = self.l_query(header.n_vars - header.n_public)?;
        let h_query = self.h_query(header.domain_size as usize)?;

        let vk = VerifyingKey::<Bn254> {
            alpha_g1: header.verifying_key.alpha_g1,
            beta_g2: header.verifying_key.beta_g2,
            gamma_g2: header.verifying_key.gamma_g2,
            delta_g2: header.verifying_key.delta_g2,
            gamma_abc_g1: ic,
        };

        let pk = ProvingKey::<Bn254> {
            vk,
            beta_g1: header.verifying_key.beta_g1,
            delta_g1: header.verifying_key.delta_g1,
            a_query,
            b_g1_query,
            b_g2_query,
            h_query,
            l_query,
        };

        Ok(pk)
    }

    fn get_section(&self, id: u32) -> Section {
        self.sections.get(&id).unwrap()[0].clone()
    }

    pub fn groth_header(&mut self) -> IoResult<HeaderGroth> {
        let section = self.get_section(2);
        let header = HeaderGroth::new(&mut self.reader, &section)?;
        Ok(header)
    }

    pub fn ic(&mut self, n_public: usize) -> IoResult<Vec<G1Affine>> {
        let ic_no_one = self.g1_section(n_public, 3)?;
        let mut ic = vec![G1Affine::new(Fq::one(), Fq::one(), false)];
        ic.extend_from_slice(&ic_no_one);
        Ok(ic)
    }

    // Section 4 is the coefficients, we ignore it

    pub fn a_query(&mut self, n_vars: usize) -> IoResult<Vec<G1Affine>> {
        self.g1_section(n_vars, 5)
    }

    pub fn b_g1_query(&mut self, n_vars: usize) -> IoResult<Vec<G1Affine>> {
        self.g1_section(n_vars, 6)
    }

    pub fn b_g2_query(&mut self, n_vars: usize) -> IoResult<Vec<G2Affine>> {
        self.g2_section(n_vars, 7)
    }

    pub fn l_query(&mut self, n_vars: usize) -> IoResult<Vec<G1Affine>> {
        self.g1_section(n_vars, 8)
    }

    pub fn h_query(&mut self, n_vars: usize) -> IoResult<Vec<G1Affine>> {
        self.g1_section(n_vars, 9)
    }

    fn g1_section(&mut self, num: usize, section_id: usize) -> IoResult<Vec<G1Affine>> {
        let section = self.get_section(section_id as u32);
        deserialize_g1_vec(
            &self.reader.get_ref()[section.position as usize..],
            num as u32,
        )
    }

    fn g2_section(&mut self, num: usize, section_id: usize) -> IoResult<Vec<G2Affine>> {
        let section = self.get_section(section_id as u32);
        deserialize_g2_vec(
            &self.reader.get_ref()[section.position as usize..],
            num as u32,
        )
    }
}

#[derive(Default, Clone, Debug, CanonicalDeserialize)]
pub struct ZVerifyingKey {
    pub alpha_g1: G1Affine,
    pub beta_g1: G1Affine,
    pub beta_g2: G2Affine,
    pub gamma_g2: G2Affine,
    pub delta_g1: G1Affine,
    pub delta_g2: G2Affine,
}

impl ZVerifyingKey {
    fn new<R: Read>(reader: &mut R) -> IoResult<Self> {
        Ok(Self {
            alpha_g1: deserialize_g1(reader)?,
            beta_g1: deserialize_g1(reader)?,
            beta_g2: deserialize_g2(reader)?,
            gamma_g2: deserialize_g2(reader)?,
            delta_g1: deserialize_g1(reader)?,
            delta_g2: deserialize_g2(reader)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct HeaderGroth {
    pub n8q: u32,
    pub q: BigInteger256,

    pub n8r: u32,
    pub r: BigInteger256,

    pub n_vars: usize,
    pub n_public: usize,

    pub domain_size: u32,
    pub power: u32,

    pub verifying_key: ZVerifyingKey,
}

impl HeaderGroth {
    pub fn new(reader: &mut Cursor<&[u8]>, section: &Section) -> IoResult<Self> {
        reader.set_position(section.position);
        Self::read(reader)
    }

    fn read<R: Read>(mut reader: &mut R) -> IoResult<Self> {
        // TODO: Impl From<u32> in Arkworks
        let n8q: u32 = FromBytes::read(&mut reader)?;
        // group order r of Bn254
        let q = BigInteger256::read(&mut reader)?;

        let n8r: u32 = FromBytes::read(&mut reader)?;
        // Prime field modulus
        let r = BigInteger256::read(&mut reader)?;

        let n_vars = u32::read(&mut reader)? as usize;
        let n_public = u32::read(&mut reader)? as usize;

        let domain_size: u32 = FromBytes::read(&mut reader)?;
        let power = log2(domain_size as usize);

        let verifying_key = ZVerifyingKey::new(&mut reader)?;

        Ok(Self {
            n8q,
            q,
            n8r,
            r,
            n_vars,
            n_public,
            domain_size,
            power,
            verifying_key,
        })
    }
}

// skips the multiplication by R because Circom points are already in Montgomery form
fn deserialize_field<R: Read>(reader: &mut R) -> IoResult<Fq> {
    let bigint = BigInteger256::read(reader)?;
    Ok(Fq::new(bigint))
}

fn deserialize_g1<R: Read>(reader: &mut R) -> IoResult<G1Affine> {
    let x = deserialize_field(reader)?;
    let y = deserialize_field(reader)?;
    Ok(G1Affine::new(x, y, false))
}

fn deserialize_g2<R: Read>(reader: &mut R) -> IoResult<G2Affine> {
    let c0 = deserialize_field(reader)?;
    let c1 = deserialize_field(reader)?;
    let f1 = Fq2::new(c0, c1);

    let c0 = deserialize_field(reader)?;
    let c1 = deserialize_field(reader)?;
    let f2 = Fq2::new(c0, c1);
    Ok(G2Affine::new(f1, f2, false))
}

fn deserialize_g1_vec(buf: &[u8], n_vars: u32) -> IoResult<Vec<G1Affine>> {
    let size = G1Affine::zero().uncompressed_size();
    let mut v = vec![];
    for i in 0..n_vars as usize {
        let el = deserialize_g1(&mut &buf[i * size..(i + 1) * size])?;
        v.push(el);
    }
    Ok(v)
}

fn deserialize_g2_vec(buf: &[u8], n_vars: u32) -> IoResult<Vec<G2Affine>> {
    let size = G2Affine::zero().uncompressed_size();
    let mut v = vec![];
    for i in 0..n_vars as usize {
        let el = deserialize_g2(&mut &buf[i * size..(i + 1) * size])?;
        v.push(el);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{G1Projective, G2Projective};
    use memmap::*;
    use num_bigint::BigUint;
    use std::fs::File;

    use crate::{CircomBuilder, CircuitConfig};
    use ark_groth16::{create_random_proof as prove, prepare_verifying_key, verify_proof};
    use ark_std::rand::thread_rng;
    use num_traits::{One, Zero};
    use std::str::FromStr;

    use std::convert::TryFrom;

    fn fq_from_str(s: &str) -> Fq {
        BigInteger256::try_from(BigUint::from_str(s).unwrap())
            .unwrap()
            .into()
    }

    // Circom snarkjs code:
    // console.log(curve.G1.F.one)
    fn fq_buf() -> Vec<u8> {
        vec![
            157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121,
            120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14,
        ]
    }

    // Circom snarkjs code:
    // const buff = new Uint8Array(curve.G1.F.n8*2);
    // curve.G1.toRprLEM(buff, 0, curve.G1.one);
    // console.dir( buff, { 'maxArrayLength': null })
    fn g1_buf() -> Vec<u8> {
        vec![
            157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121,
            120, 111, 163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14, 58, 27, 30, 139, 27, 135,
            186, 166, 123, 22, 142, 235, 81, 214, 241, 20, 88, 140, 242, 240, 222, 70, 221, 204,
            94, 190, 15, 52, 131, 239, 20, 28,
        ]
    }

    // Circom snarkjs code:
    // const buff = new Uint8Array(curve.G2.F.n8*2);
    // curve.G2.toRprLEM(buff, 0, curve.G2.one);
    // console.dir( buff, { 'maxArrayLength': null })
    fn g2_buf() -> Vec<u8> {
        vec![
            38, 32, 188, 2, 209, 181, 131, 142, 114, 1, 123, 73, 53, 25, 235, 220, 223, 26, 129,
            151, 71, 38, 184, 251, 59, 80, 150, 175, 65, 56, 87, 25, 64, 97, 76, 168, 125, 115,
            180, 175, 196, 216, 2, 88, 90, 221, 67, 96, 134, 47, 160, 82, 252, 80, 233, 9, 107,
            123, 234, 58, 131, 240, 254, 20, 246, 233, 107, 136, 157, 250, 157, 97, 120, 155, 158,
            245, 151, 210, 127, 254, 254, 125, 27, 35, 98, 26, 158, 255, 6, 66, 158, 174, 235, 126,
            253, 40, 238, 86, 24, 199, 86, 91, 9, 100, 187, 60, 125, 50, 34, 249, 87, 220, 118, 16,
            53, 51, 190, 53, 249, 85, 130, 100, 253, 147, 230, 160, 164, 13,
        ]
    }

    // Circom logs in Projective coordinates: console.log(curve.G1.one)
    fn g1_one() -> G1Affine {
        let x = Fq::one();
        let y = Fq::one() + Fq::one();
        let z = Fq::one();
        G1Affine::from(G1Projective::new(x, y, z))
    }

    // Circom logs in Projective coordinates: console.log(curve.G2.one)
    fn g2_one() -> G2Affine {
        let x = Fq2::new(
            fq_from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            ),
            fq_from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            ),
        );

        let y = Fq2::new(
            fq_from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            ),
            fq_from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            ),
        );
        let z = Fq2::new(Fq::one(), Fq::zero());
        G2Affine::from(G2Projective::new(x, y, z))
    }

    #[test]
    fn can_deser_fq() {
        let buf = fq_buf();
        let fq = deserialize_field(&mut &buf[..]).unwrap();
        assert_eq!(fq, Fq::one());
    }

    #[test]
    fn can_deser_g1() {
        let buf = g1_buf();
        assert_eq!(buf.len(), 64);
        let g1 = deserialize_g1(&mut &buf[..]).unwrap();
        let expected = g1_one();
        assert_eq!(g1, expected);
    }

    #[test]
    fn can_deser_g1_vec() {
        let n_vars = 10;
        let buf = vec![g1_buf(); n_vars]
            .iter()
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        let expected = vec![g1_one(); n_vars];

        let de = deserialize_g1_vec(&buf[..], n_vars as u32).unwrap();
        assert_eq!(expected, de);
    }

    #[test]
    fn can_deser_g2() {
        let buf = g2_buf();
        assert_eq!(buf.len(), 128);
        let g2 = deserialize_g2(&mut &buf[..]).unwrap();

        let expected = g2_one();
        assert_eq!(g2, expected);
    }

    #[test]
    fn can_deser_g2_vec() {
        let n_vars = 10;
        let buf = vec![g2_buf(); n_vars]
            .iter()
            .cloned()
            .flatten()
            .collect::<Vec<_>>();
        let expected = vec![g2_one(); n_vars];

        let de = deserialize_g2_vec(&buf[..], n_vars as u32).unwrap();
        assert_eq!(expected, de);
    }

    #[test]
    fn header() {
        // `circom --r1cs` using the below file:
        //
        //  template Multiplier() {
        //     signal private input a;
        //     signal private input b;
        //     signal output c;
        //
        //     c <== a*b;
        // }
        //
        // component main = Multiplier();
        //
        // Then:
        // `snarkjs zkey new circuit.r1cs powersOfTau28_hez_final_10.ptau test.zkey`
        let path = "./test-vectors/test.zkey";
        let file = File::open(path).unwrap();
        let map = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("unable to create a memory map")
        };
        let mut reader = Cursor::new(map.as_ref());
        let mut binfile = BinFile::new(&mut reader).unwrap();
        let header = binfile.groth_header().unwrap();
        assert_eq!(header.n_vars, 4);
        assert_eq!(header.n_public, 1);
        assert_eq!(header.domain_size, 2);
        assert_eq!(header.power, 2);
    }

    #[test]
    fn deser_ic() {
        let path = "./test-vectors/test.zkey";
        let file = File::open(path).unwrap();
        let map = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("unable to create a memory map")
        };
        let mut reader = Cursor::new(map.as_ref());
        let mut binfile = BinFile::new(&mut reader).unwrap();
        let params = binfile.proving_key().unwrap();

        let ic = params.vk.gamma_abc_g1;
        // TODO: read the IC from Circom and see if it matches
    }

    #[test]
    fn verify_proof_with_zkey() {
        let path = "./test-vectors/test.zkey";
        let file = File::open(path).unwrap();
        let map = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("unable to create a memory map")
        };
        let mut reader = Cursor::new(map.as_ref());
        let mut binfile = BinFile::new(&mut reader).unwrap();

        let params = binfile.proving_key().unwrap();
        dbg!(params.vk.gamma_abc_g1.len());

        let cfg = CircuitConfig::<Bn254>::new(
            "./test-vectors/mycircuit.wasm",
            "./test-vectors/mycircuit.r1cs",
        )
        .unwrap();
        let mut builder = CircomBuilder::new(cfg);
        builder.push_input("a", 3);
        builder.push_input("b", 11);

        let circom = builder.build().unwrap();

        let inputs = circom.get_public_inputs().unwrap();
        dbg!(inputs.len());

        let mut rng = thread_rng();
        let proof = prove(circom, &params, &mut rng).unwrap();

        let pvk = prepare_verifying_key(&params.vk);

        let verified = verify_proof(&pvk, &proof, &inputs).unwrap();

        assert!(verified);
    }
}
