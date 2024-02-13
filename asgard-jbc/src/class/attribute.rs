use bitflags::bitflags;

use crate::{Decode, Result};

pub struct MethodParametersAttribute(pub Vec<MethodParameter>);

impl Decode for MethodParametersAttribute {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let parameters_count = u16::decode(input)?;
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        for _ in 0..parameters_count {
            parameters.push(Decode::decode(input)?);
        }
        Ok(Self(parameters))
    }
}

pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: MethodParameterAccessFlags,
}

impl Decode for MethodParameter {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let name_index = Decode::decode(input)?;
        let access_flags = MethodParameterAccessFlags::from_bits(u16::decode(input)?).unwrap();
        Ok(Self {
            name_index,
            access_flags,
        })
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct MethodParameterAccessFlags: u16 {
        const FINAL = 0x0010;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}
