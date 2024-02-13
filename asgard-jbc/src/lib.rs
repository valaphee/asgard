use std::io::{Read, Seek, Write};

use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use thiserror::Error;

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

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for ClassFile {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let magic = u32::decode(input)?; // 0xCAFEBABE
        let minor_version = Decode::decode(input)?;
        let major_version = Decode::decode(input)?;
        let constant_pool_count = u16::decode(input)? - 1;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);
        for _ in 0..constant_pool_count {
            constant_pool.push(ConstantPoolInfo::decode(input)?);
        }
        let access_flags = AccessFlags::from_bits(Decode::decode(input)?).unwrap();
        let this_class = Decode::decode(input)?;
        let super_class = Decode::decode(input)?;
        let interfaces_count = u16::decode(input)?;
        let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(Decode::decode(input)?);
        }
        
        let fields_count = u16::decode(input)?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(Decode::decode(input)?);
        }
        let methods_count = u16::decode(input)?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Decode::decode(input)?);
        }
        let attributes_count = u16::decode(input)?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Decode::decode(input)?);
        }
        Ok(Self {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

#[derive(Debug)]
pub enum ConstantPoolInfo {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class {
        name_index: u16,
    },
    String {
        string_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

impl Decode for ConstantPoolInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let tag = input.read_u8()?;
        let info = match tag {
            1 => {
                let length = u16::decode(input)?;
                let mut bytes = vec![0; length as usize];
                input.read_exact(&mut bytes)?;
                ConstantPoolInfo::Utf8(String::from_utf8(bytes).unwrap())
            }
            3 => ConstantPoolInfo::Integer(Decode::decode(input)?),
            4 => ConstantPoolInfo::Float(Decode::decode(input)?),
            5 => ConstantPoolInfo::Long(Decode::decode(input)?),
            6 => ConstantPoolInfo::Double(Decode::decode(input)?),
            7 => ConstantPoolInfo::Class {
                name_index: Decode::decode(input)?,
            },
            8 => ConstantPoolInfo::String {
                string_index: Decode::decode(input)?,
            },
            9 => ConstantPoolInfo::Fieldref {
                class_index: Decode::decode(input)?,
                name_and_type_index: Decode::decode(input)?,
            },
            10 => ConstantPoolInfo::Methodref {
                class_index: Decode::decode(input)?,
                name_and_type_index: Decode::decode(input)?,
            },
            11 => ConstantPoolInfo::InterfaceMethodref {
                class_index: Decode::decode(input)?,
                name_and_type_index: Decode::decode(input)?,
            },
            12 => ConstantPoolInfo::NameAndType {
                name_index: Decode::decode(input)?,
                descriptor_index: Decode::decode(input)?,
            },
            15 => ConstantPoolInfo::MethodHandle {
                reference_kind: Decode::decode(input)?,
                reference_index: Decode::decode(input)?,
            },
            16 => ConstantPoolInfo::MethodType {
                descriptor_index: Decode::decode(input)?,
            },
            17 => ConstantPoolInfo::Dynamic {
                bootstrap_method_attr_index: Decode::decode(input)?,
                name_and_type_index: Decode::decode(input)?,
            },
            18 => ConstantPoolInfo::InvokeDynamic {
                bootstrap_method_attr_index: Decode::decode(input)?,
                name_and_type_index: Decode::decode(input)?,
            },
            19 => ConstantPoolInfo::Module {
                name_index: Decode::decode(input)?,
            },
            20 => ConstantPoolInfo::Package {
                name_index: Decode::decode(input)?,
            },
            _ => unreachable!()
        };
        Ok(info)
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for FieldInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let access_flags = AccessFlags::from_bits(Decode::decode(input)?).unwrap();
        let name_index = Decode::decode(input)?;
        let descriptor_index = Decode::decode(input)?;
        let attributes_count = u16::decode(input)?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Decode::decode(input)?);
        }
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for MethodInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let access_flags = AccessFlags::from_bits(Decode::decode(input)?).unwrap();
        let name_index = Decode::decode(input)?;
        let descriptor_index = Decode::decode(input)?;
        let attributes_count = u16::decode(input)?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Decode::decode(input)?);
        }
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub info: Vec<u8>
}

impl Decode for AttributeInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let attribute_name_index = Decode::decode(input)?;
        let attribute_length = u32::decode(input)?;
        let mut info = Vec::with_capacity(attribute_length as usize);
        for _ in 0..attribute_length {
            info.push(Decode::decode(input)?);
        }
        Ok(Self {
            attribute_name_index,
            info,
        })
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct AccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SUPER_SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const INTERFACE = 0x0200;        
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }
}
