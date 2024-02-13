use std::str::FromStr;

use crate::{Error, Result};

#[derive(Debug)]
pub enum FieldType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Object(String),
    Short,
    Boolean,
    Array(Box<Self>),
    Void,
}

impl FieldType {
    fn from_str_internal(s: &mut &str) -> Result<Self> {
        let field_type = s.chars().next().unwrap();
        *s = &s[1..];
        Ok(match field_type {
            'B' => Self::Byte,
            'C' => Self::Char,
            'D' => Self::Double,
            'F' => Self::Float,
            'I' => Self::Int,
            'J' => Self::Long,
            'L' => {
                let class_name_end = s.find(';').unwrap();
                *s = &s[class_name_end + 1..];
                let class_name = &s[..class_name_end];
                Self::Object(class_name.to_owned())
            },
            'S' => Self::Short,
            'Z' => Self::Boolean,
            '[' => Self::Array(Box::new(s.parse()?)),
            'V' => Self::Void,
            _ => todo!()
        })
    }
}

impl FromStr for FieldType {
    type Err = Error;

    fn from_str(mut s: &str) -> Result<Self> {
        Self::from_str_internal(&mut s)
    }
}

#[derive(Debug)]
pub struct MethodDescriptor {
    pub parameter_types: Vec<FieldType>,
    pub return_type: FieldType,
}

impl FromStr for MethodDescriptor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parameter_descriptor_begin = s.find('(').unwrap();
        let parameter_descriptor_end = s.find(')').unwrap();
        let mut parameter_descriptor = &s[parameter_descriptor_begin + 1..parameter_descriptor_end];
        let mut parameter_types = vec![];
        while !parameter_descriptor.is_empty() {
            parameter_types.push(FieldType::from_str_internal(&mut parameter_descriptor)?);
        }
        let return_descriptor = &s[parameter_descriptor_end + 1..];
        let return_type = return_descriptor.parse()?;
        Ok(Self {
            parameter_types,
            return_type,
        })
    }
}
