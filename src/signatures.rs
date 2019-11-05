// TODO: signatures still need a bit of work and testing
#![allow(dead_code)]

use crate::codes::*;
use crate::database::*;
use crate::tables::*;
use std::io::Result;
use std::vec::*;

// TODO: what about using std::io::Read?

pub struct GenericSig<'a> {
    sig_type: TypeDefOrRef<'a>,
    args: Vec<TypeSig<'a>>,
}

impl<'a> GenericSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<GenericSig<'a>> {
        let (_, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);

        let (code, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);
        let sig_type = TypeDefOrRef::decode(db, code)?;

        let (arg_count, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);

        let mut args = Vec::with_capacity(arg_count as usize);

        for _ in 0..arg_count {
            args.push(TypeSig::new(db, bytes)?);
        }

        Ok(GenericSig { sig_type, args })
    }

    pub fn sig_type(&self) -> &TypeDefOrRef<'a> {
        &self.sig_type
    }

    pub fn args(&self) -> &Vec<TypeSig<'a>> {
        &self.args
    }
}

impl<'a> std::fmt::Display for GenericSig<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sig_type)
    }
}

pub struct ModifierSig<'a> {
    sig_type: TypeDefOrRef<'a>,
}

impl<'a> ModifierSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<ModifierSig<'a>> {
        let (_, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);
        let (code, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);
        let sig_type = TypeDefOrRef::decode(db, code)?;
        Ok(ModifierSig { sig_type })
    }

    fn vec(db: &'a Database, bytes: &mut &[u8]) -> Result<Vec<ModifierSig<'a>>> {
        let mut modifiers = Vec::new();
        loop {
            let (element_type, _) = read_u32(bytes)?;
            if element_type != 32 && element_type != 31 {
                break;
            } else {
                modifiers.push(ModifierSig::new(db, bytes)?);
            }
        }
        Ok(modifiers)
    }
}

pub struct MethodSig<'a> {
    return_type: Option<TypeSig<'a>>,
    params: Vec<(Param<'a>, ParamSig<'a>)>,
}

impl<'a> MethodSig<'a> {
    pub(crate) fn new(method: &MethodDef<'a>) -> Result<MethodSig<'a>> {
        let mut bytes = method.row.blob(4)?;
        let (calling_convention, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);
        if calling_convention & 0x10 != 0 {
            let (_, bytes_read) = read_u32(&mut bytes)?;
            bytes = seek(bytes, bytes_read);
        }
        let (param_count, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);
        ModifierSig::vec(method.row.table.db, &mut bytes)?;
        read_expected(&mut bytes, 0x10)?;
        let return_type = if read_expected(&mut bytes, 0x01)? { None } else { Some(TypeSig::new(method.row.table.db, &mut bytes)?) };
        let mut params = Vec::with_capacity(param_count as usize);
        for param in method.params()? {
            if !return_type.is_some() || param.sequence()? != 0 {
                params.push((param, ParamSig::new(method.row.table.db, &mut bytes)?));
            }
        }
        Ok(MethodSig { return_type, params })
    }

    pub fn return_type(&self) -> &Option<TypeSig<'a>> {
        &self.return_type
    }

    pub fn params(&self) -> &Vec<(Param<'a>, ParamSig<'a>)> {
        &self.params
    }
}

pub struct ParamSig<'a> {
    modifiers: Vec<ModifierSig<'a>>,
    by_ref: bool,
    sig_type: TypeSig<'a>,
}

impl<'a> ParamSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<ParamSig<'a>> {
        let modifiers = ModifierSig::vec(db, bytes)?;
        let by_ref = read_expected(bytes, 0x10)?;
        let sig_type = TypeSig::new(db, bytes)?;
        Ok(ParamSig { modifiers, by_ref, sig_type })
    }

    pub fn sig_type(&self) -> &TypeSig<'a> {
        &self.sig_type
    }
}

pub enum TypeSigType<'a> {
    ElementType(ElementType),
    TypeDefOrRef(TypeDefOrRef<'a>),
    GenericSig(GenericSig<'a>),
    GenericTypeIndex(u32),
    GenericMethodIndex(u32),
}

impl<'a> TypeSigType<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<TypeSigType<'a>> {
        let (element_type, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);

        Ok(match element_type {
            0x02 => TypeSigType::ElementType(ElementType::Bool),
            0x03 => TypeSigType::ElementType(ElementType::Char),
            0x04 => TypeSigType::ElementType(ElementType::I8),
            0x05 => TypeSigType::ElementType(ElementType::U8),
            0x06 => TypeSigType::ElementType(ElementType::I16),
            0x07 => TypeSigType::ElementType(ElementType::U16),
            0x08 => TypeSigType::ElementType(ElementType::I32),
            0x09 => TypeSigType::ElementType(ElementType::U32),
            0x0A => TypeSigType::ElementType(ElementType::I64),
            0x0B => TypeSigType::ElementType(ElementType::U64),
            0x0C => TypeSigType::ElementType(ElementType::F32),
            0x0D => TypeSigType::ElementType(ElementType::F64),
            0x0E => TypeSigType::ElementType(ElementType::String),
            0x1C => TypeSigType::ElementType(ElementType::Object),
            0x11 | 0x12 => {
                let (code, bytes_read) = read_u32(bytes)?;
                *bytes = seek(bytes, bytes_read);
                TypeSigType::TypeDefOrRef(TypeDefOrRef::decode(db, code)?)
            }
            0x13 => {
                let (index, bytes_read) = read_u32(bytes)?;
                *bytes = seek(bytes, bytes_read);
                TypeSigType::GenericTypeIndex(index)
            }
            0x15 => TypeSigType::GenericSig(GenericSig::new(db, bytes)?),
            0x1e => {
                let (index, bytes_read) = read_u32(bytes)?;
                *bytes = seek(bytes, bytes_read);
                TypeSigType::GenericMethodIndex(index)
            }
            _ => return Err(unsupported_blob()),
        })
    }
}

impl<'a> std::fmt::Display for TypeSigType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSigType::ElementType(value) => write!(f, "{}", value),
            TypeSigType::TypeDefOrRef(value) => write!(f, "{}", value),
            TypeSigType::GenericSig(value) => write!(f, "{}", value),
            TypeSigType::GenericTypeIndex(value) => write!(f, "{}", value),
            TypeSigType::GenericMethodIndex(value) => write!(f, "{}", value),
        }
    }
}

pub struct TypeSig<'a> {
    array: bool,
    modifiers: Vec<ModifierSig<'a>>,
    sig_type: TypeSigType<'a>,
}

impl<'a> TypeSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<TypeSig<'a>> {
        let array = read_expected(bytes, 0x1D)?;
        let modifiers = ModifierSig::vec(db, bytes)?;
        let sig_type = TypeSigType::new(db, bytes)?;
        Ok(TypeSig { array, modifiers, sig_type })
    }

    pub fn sig_type(&self) -> &TypeSigType<'a> {
        &self.sig_type
    }
}

impl<'a> std::fmt::Display for TypeSig<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sig_type)
    }
}

fn read_expected(bytes: &mut &[u8], expected: u32) -> Result<bool> {
    let (element_type, bytes_read) = read_u32(bytes)?;
    Ok(if element_type == expected {
        *bytes = seek(bytes, bytes_read);
        true
    } else {
        false
    })
}

fn seek(bytes: &[u8], bytes_read: usize) -> &[u8] {
    bytes.get(bytes_read..).unwrap()
}

fn read_u32<'a>(bytes: &[u8]) -> Result<(u32, usize)> {
    if bytes.is_empty() {
        return Err(unsupported_blob());
    }
    let (bytes_read, value) = if bytes[0] & 0x80 == 0 {
        (1, bytes[0] as u32)
    } else if bytes[0] & 0xC0 == 0x80 {
        if bytes.len() < 2 {
            return Err(unsupported_blob());
        }
        (2, (((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32)
    } else if bytes[0] & 0xE0 == 0xC0 {
        if bytes.len() < 4 {
            return Err(unsupported_blob());
        }
        (4, (((bytes[0] & 0x1F) as u32) << 24) | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32)
    } else {
        return Err(unsupported_blob());
    };

    Ok((value, bytes_read))
}

fn unsupported_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported blob")
}

pub enum ElementType {
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Object,
}

impl std::fmt::Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementType::Bool => write!(f, "bool"),
            ElementType::Char => write!(f, "char"),
            ElementType::I8 => write!(f, "i8"),
            ElementType::U8 => write!(f, "u8"),
            ElementType::I16 => write!(f, "i16"),
            ElementType::U16 => write!(f, "u16"),
            ElementType::I32 => write!(f, "i32"),
            ElementType::U32 => write!(f, "u32"),
            ElementType::I64 => write!(f, "i64"),
            ElementType::U64 => write!(f, "u64"),
            ElementType::F32 => write!(f, "f32"),
            ElementType::F64 => write!(f, "f64"),
            ElementType::String => write!(f, "String"),
            ElementType::Object => write!(f, "Object"),
        }
    }
}
