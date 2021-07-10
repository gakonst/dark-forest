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
        let ic = self.ic(header.n_public)?;

        let a_query = self.a_query(header.n_vars)?;
        let b_g1_query = self.b_g1_query(header.n_vars)?;
        let b_g2_query = self.b_g2_query(header.n_vars)?;
        let l_query = self.l_query(header.n_vars - header.n_public - 1)?;
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
        self.g1_section(n_public, 3)
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
        deserialize_g1_vec(&mut self.reader, &section, num as u32)
    }

    fn g2_section(&mut self, num: usize, section_id: usize) -> IoResult<Vec<G2Affine>> {
        let section = self.get_section(section_id as u32);
        deserialize_g2_vec(&mut self.reader, &section, num as u32)
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

fn deserialize_g1_vec(
    reader: &mut Cursor<&[u8]>,
    section: &Section,
    n_vars: u32,
) -> IoResult<Vec<G1Affine>> {
    let size = 64;
    let buf = &reader.get_ref()[section.position as usize..];
    let mut v = vec![];
    for i in 0..n_vars as usize {
        let el = deserialize_g1(&mut &buf[i * size..(i + 1) * size])?;
        v.push(el);
    }
    Ok(v)
}

fn deserialize_g2_vec(
    reader: &mut Cursor<&[u8]>,
    section: &Section,
    n_vars: u32,
) -> IoResult<Vec<G2Affine>> {
    let size = 128;
    let buf = &reader.get_ref()[section.position as usize..];
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
    use memmap::*;
    use std::fs::File;

    #[test]
    fn snarkjs_g1() {
        let buf = vec![
            23, 14, 17, 42, 185, 164, 205, 1, 195, 107, 171, 71, 64, 46, 252, 207, 233, 238, 75,
            26, 225, 17, 222, 60, 207, 94, 92, 15, 152, 2, 235, 6, 30, 139, 14, 214, 223, 44, 75,
            49, 54, 176, 41, 90, 23, 66, 228, 60, 120, 2, 126, 203, 170, 53, 127, 17, 146, 101, 59,
            78, 218, 81, 70, 6,
        ];

        // converts from montgomery to normal
        let bigint = BigInteger256::deserialize(&buf[..32]).unwrap();
        let f = Fq::new(bigint);
    }

    #[test]
    fn deser() {
        let path = "./test-vectors/test.zkey";
        let file = File::open(path).unwrap();
        let map = unsafe {
            MmapOptions::new()
                .map(&file)
                .expect("unable to create a memory map")
        };
        let mut reader = Cursor::new(map.as_ref());
        let mut binfile = BinFile::new(&mut reader).unwrap();

        let pk = binfile.proving_key().unwrap();
    }
}
