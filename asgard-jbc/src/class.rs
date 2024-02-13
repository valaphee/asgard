use std::io::Read;

use bitflags::bitflags;

use crate::{Decode, Result};

pub mod attribute;

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for ClassFile {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let _magic = u32::decode(input)?; // 0xCAFEBABE
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
        let tag = u8::decode(input)?;
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
            _ => todo!(),
        };
        Ok(info)
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for FieldInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let access_flags = FieldAccessFlags::from_bits(Decode::decode(input)?).unwrap();
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

bitflags! {
    #[derive(Debug)]
    pub struct FieldAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const VOLATILE = 0x0040;
        const TRANSIENT = 0x0080;
        const SYNTHETIC = 0x1000;
        const ENUM = 0x4000;
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl Decode for MethodInfo {
    fn decode(input: &mut &[u8]) -> Result<Self> {
        let access_flags = MethodAccessFlags::from_bits(Decode::decode(input)?).unwrap();
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

bitflags! {
    #[derive(Debug)]
    pub struct MethodAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
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
