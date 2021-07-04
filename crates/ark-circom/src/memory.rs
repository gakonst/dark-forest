use wasmer::{Memory, MemoryView};

// TODO: Decide whether we want Ark here or if it should use a generic BigInt package
use ark_bn254::FrParameters;
use ark_ff::{BigInteger, BigInteger256 as BigInt, FpParameters, FromBytes};


use color_eyre::Result;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct SafeMem {
    pub memory: Memory,

    short_max: BigInt,
    short_min: BigInt,
}

impl Deref for SafeMem {
    type Target = Memory;

    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

impl SafeMem {
    pub fn new(memory: Memory) -> Self {
        let short_max = BigInt::from(0x80000000u64);
        let mut short_min = FrParameters::MODULUS;
        short_min.sub_noborrow(&short_max);

        Self {
            memory,
            short_min,
            short_max,
        }
    }

    pub fn view(&self) -> MemoryView<u32> {
        self.memory.view()
    }

    pub fn free_pos(&self) -> u32 {
        self.view()[0].get()
    }

    pub fn set_free_pos(&mut self, ptr: u32) {
        self.write_u32(0, ptr);
    }

    pub fn alloc_u32(&mut self) -> u32 {
        let p = self.free_pos();
        self.set_free_pos(p + 8);
        p
    }

    /// Writes a `num` to the provided position of the buffer
    ///
    /// This is marked as `&mut self` for safety
    pub fn write_u32(&mut self, ptr: usize, num: u32) {
        let buf = unsafe { self.memory.data_unchecked_mut() };
        buf[ptr..ptr + std::mem::size_of::<u32>()].copy_from_slice(&num.to_le_bytes());
    }

    /// Reads a u32 from the specific slice
    pub fn read_u32(&self, ptr: usize) -> u32 {
        let buf = unsafe { self.memory.data_unchecked() };

        let mut bytes = [0; 4];
        bytes.copy_from_slice(&buf[ptr..ptr + std::mem::size_of::<u32>()]);

        u32::from_le_bytes(bytes)
    }

    pub fn alloc_fr(&mut self) -> u32 {
        let n32 = 8;
        let p = self.free_pos();
        self.set_free_pos(p + n32 * 4 + 8);
        p
    }

    pub fn read_big(&self, ptr: usize, num_bytes: usize) -> Result<BigInt> {
        let buf = unsafe { self.memory.data_unchecked() };
        let buf = &buf[ptr..ptr + num_bytes * 32];
        Ok(BigInt::read(buf).unwrap())
    }

    pub fn write_big(&self, ptr: usize, num: BigInt) -> Result<()> {
        let buf = unsafe { self.memory.data_unchecked_mut() };
        let bytes = num.to_bytes_le();
        let len = bytes.len();

        buf[ptr..ptr + len].copy_from_slice(&bytes);
        Ok(())
    }

    pub fn write_fr(&mut self, ptr: usize, fr: BigInt) -> Result<()> {
        if fr < self.short_max {
            self.write_short_positive(ptr, fr)?;
        } else if fr > self.short_min {
            self.write_short_negative(ptr, fr)?;
        } else {
            self.write_long_normal(ptr, fr)?;
        }

        Ok(())
    }

    fn write_short_positive(&mut self, ptr: usize, fr: BigInt) -> Result<()> {
        let num = fr.0[0] as u32;
        self.write_u32(ptr, num);
        self.write_u32(ptr + 4, 0);
        Ok(())
    }

    fn write_short_negative(&mut self, ptr: usize, mut fr: BigInt) -> Result<()> {
        // prime
        let mut v_neg = FrParameters::MODULUS;
        // prime - max
        v_neg.sub_noborrow(&self.short_max);
        // fr - (prime - max)
        fr.sub_noborrow(&v_neg);
        // max + (v - (prime - max))
        let mut res = self.short_max;
        res.add_nocarry(&fr);

        self.write_u32(ptr, res.0[0] as u32);
        self.write_u32(ptr + 4, 0);
        Ok(())
    }

    fn write_long_normal(&mut self, ptr: usize, fr: BigInt) -> Result<()> {
        self.write_u32(ptr, 0);
        self.write_u32(ptr + 4, i32::MIN as u32);
        self.write_big(ptr + 8, fr)?;
        Ok(())
    }
}
