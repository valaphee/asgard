use std::io::{Seek, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

pub mod class;
pub mod descriptor;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Encode {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()>;
}

pub trait Decode: Sized {
    fn decode(input: &mut &[u8]) -> Result<Self>;
}

impl Encode for u8 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_u8(*self)?;
        Ok(())
    }
}

impl Decode for u8 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_u8()?)
    }
}

impl Encode for u16 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for u16 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_u16::<BigEndian>()?)
    }
}

impl Encode for u32 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for u32 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_u32::<BigEndian>()?)
    }
}

impl Encode for i32 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for i32 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_i32::<BigEndian>()?)
    }
}

impl Encode for u64 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for u64 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_u64::<BigEndian>()?)
    }
}

impl Encode for i64 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for i64 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_i64::<BigEndian>()?)
    }
}

impl Encode for f32 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_f32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for f32 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_f32::<BigEndian>()?)
    }
}

impl Encode for f64 {
    fn encode(&self, output: &mut (impl Write + Seek)) -> Result<()> {
        output.write_f64::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Decode for f64 {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        Ok(input.read_f64::<BigEndian>()?)
    }
}
