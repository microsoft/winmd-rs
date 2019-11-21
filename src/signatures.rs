// TODO: signatures still need a bit of work and testing
#![allow(dead_code)]

use crate::codes::*;
use crate::error::*;
use crate::file::*;
use crate::tables::*;
use std::convert::*;
use std::vec::*;

pub struct GenericSig<'a> {
    sig_type: TypeDefOrRef<'a>,
    args: Vec<TypeSig<'a>>,
}

impl<'a> GenericSig<'a> {
    fn new(file: &'a File, bytes: &mut &[u8]) -> ParseResult<GenericSig<'a>> {
        read_unsigned(bytes)?;
        let sig_type = TypeDefOrRef::decode(file, read_unsigned(bytes)?)?;
        let arg_count = read_unsigned(bytes)?;
        let mut args = Vec::with_capacity(arg_count as usize);

        for _ in 0..arg_count {
            args.push(TypeSig::new(file, bytes)?);
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
    fn new(file: &'a File, bytes: &mut &[u8]) -> ParseResult<ModifierSig<'a>> {
        read_unsigned(bytes)?;
        let sig_type = TypeDefOrRef::decode(file, read_unsigned(bytes)?)?;
        Ok(ModifierSig { sig_type })
    }

    fn vec(file: &'a File, bytes: &mut &[u8]) -> ParseResult<Vec<ModifierSig<'a>>> {
        let mut modifiers = Vec::new();
        loop {
            let (element_type, _) = peek_unsigned(bytes)?;
            if element_type != 32 && element_type != 31 {
                break;
            } else {
                modifiers.push(ModifierSig::new(file, bytes)?);
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
    pub(crate) fn new(method: &MethodDef<'a>) -> ParseResult<MethodSig<'a>> {
        let mut bytes = method.row.blob(4)?;
        let calling_convention = read_unsigned(&mut bytes)?;
        if calling_convention & 0x10 != 0 {
            read_unsigned(&mut bytes)?;
        }
        let param_count = read_unsigned(&mut bytes)?;
        ModifierSig::vec(method.row.table.file, &mut bytes)?;
        read_expected(&mut bytes, 0x10)?;
        let return_type = if read_expected(&mut bytes, 0x01)? { None } else { Some(TypeSig::new(method.row.table.file, &mut bytes)?) };
        let mut params = Vec::with_capacity(param_count as usize);
        for param in method.params()? {
            if !return_type.is_some() || param.sequence()? != 0 {
                params.push((param, ParamSig::new(method.row.table.file, &mut bytes)?));
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

pub(crate) fn constructor_sig<'a>(file: &'a File, mut bytes: &[u8]) -> ParseResult<Vec<ParamSig<'a>>> {
    let calling_convention = read_unsigned(&mut bytes)?;
    if calling_convention & 0x10 != 0 {
        read_unsigned(&mut bytes)?;
    }
    let param_count = read_unsigned(&mut bytes)?;
    ModifierSig::vec(file, &mut bytes)?;
    read_expected(&mut bytes, 0x10)?;
    if !read_expected(&mut bytes, 0x01)? {
        TypeSig::new(file, &mut bytes)?;
    };
    let mut params = Vec::with_capacity(param_count as usize);
    for _ in 0..param_count {
        params.push(ParamSig::new(file, &mut bytes)?);
    }
    Ok(params)
}

#[derive(PartialEq)]
pub enum ArgumentSig<'a> {
    Bool(bool),
    Char(char),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(&'a str),
    Type(TypeDef<'a>),
}

impl<'a> std::fmt::UpperHex for ArgumentSig<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentSig::Bool(value) => write!(f, "{}", value),
            ArgumentSig::Char(value) => write!(f, "{}", value),
            ArgumentSig::I8(value) => write!(f, "{:X}", value),
            ArgumentSig::U8(value) => write!(f, "{:X}", value),
            ArgumentSig::I16(value) => write!(f, "{:X}", value),
            ArgumentSig::U16(value) => write!(f, "{:X}", value),
            ArgumentSig::I32(value) => write!(f, "{:X}", value),
            ArgumentSig::U32(value) => write!(f, "{:X}", value),
            ArgumentSig::I64(value) => write!(f, "{:X}", value),
            ArgumentSig::U64(value) => write!(f, "{:X}", value),
            ArgumentSig::F32(value) => write!(f, "{}", value),
            ArgumentSig::F64(value) => write!(f, "{}", value),
            ArgumentSig::String(value) => write!(f, "{}", value),
            ArgumentSig::Type(value) => write!(f, "{}.{}", value.namespace()?, value.name()?),
        }
    }
}

impl<'a> ArgumentSig<'a> {
    pub(crate) fn new(file: &'a File, signature_bytes: &[u8], mut data_bytes: &'a [u8]) -> ParseResult<Vec<(&'a str, ArgumentSig<'a>)>> {
        let params = constructor_sig(file, signature_bytes)?;
        read_u16(&mut data_bytes);
        let mut args = Vec::with_capacity(params.len());

        for param in params {
            args.push((
                "",
                match param.sig_type.sig_type {
                    TypeSigType::ElementType(value) => {
                        match value {
                            //ElementType::Bool =>
                            // ElementType::Char,
                            ElementType::I8 => ArgumentSig::I8(read_i8(&mut data_bytes)),
                            ElementType::U8 => ArgumentSig::U8(read_u8(&mut data_bytes)),
                            ElementType::I16 => ArgumentSig::I16(read_i16(&mut data_bytes)),
                            ElementType::U16 => ArgumentSig::U16(read_u16(&mut data_bytes)),
                            ElementType::I32 => ArgumentSig::I32(read_i32(&mut data_bytes)),
                            ElementType::U32 => ArgumentSig::U32(read_u32(&mut data_bytes)),
                            ElementType::I64 => ArgumentSig::I64(read_i64(&mut data_bytes)),
                            ElementType::U64 => ArgumentSig::U64(read_u64(&mut data_bytes)),
                            // ElementType::F32,
                            // ElementType::F64,
                            // ElementType::String,
                            _ => return Err(unsupported_blob()),
                        }
                    }
                    // TypeSigType::TypeDefOrRef(value) => {

                    // }
                    _ => return Err(unsupported_blob()),
                },
            ));
        }

        let named_args = read_u16(&mut data_bytes);

        for _ in 0..named_args {
            read_u8(&mut data_bytes);
            let arg_type = read_u8(&mut data_bytes);

            args.push(match arg_type {
                0x50 => (read_string(&mut data_bytes), ArgumentSig::String(read_string(&mut data_bytes))),
                // 0x55 => { // Enum

                // },
                2 => (read_string(&mut data_bytes), ArgumentSig::Bool(read_u8(&mut data_bytes) != 0)),
                8 => (read_string(&mut data_bytes), ArgumentSig::I32(read_i32(&mut data_bytes))),
                14 => (read_string(&mut data_bytes), ArgumentSig::String(read_string(&mut data_bytes))),
                _ => return Err(unsupported_blob()),
            });
        }

        Ok(args)
    }
}

pub struct ParamSig<'a> {
    modifiers: Vec<ModifierSig<'a>>,
    by_ref: bool,
    sig_type: TypeSig<'a>,
}

impl<'a> ParamSig<'a> {
    fn new(file: &'a File, bytes: &mut &[u8]) -> ParseResult<ParamSig<'a>> {
        let modifiers = ModifierSig::vec(file, bytes)?;
        let by_ref = read_expected(bytes, 0x10)?;
        let sig_type = TypeSig::new(file, bytes)?;
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
    fn new(file: &'a File, bytes: &mut &[u8]) -> ParseResult<TypeSigType<'a>> {
        let element_type = read_unsigned(bytes)?;

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
            0x11 | 0x12 => TypeSigType::TypeDefOrRef(TypeDefOrRef::decode(file, read_unsigned(bytes)?)?),
            0x13 => TypeSigType::GenericTypeIndex(read_unsigned(bytes)?),
            0x15 => TypeSigType::GenericSig(GenericSig::new(file, bytes)?),
            0x1e => TypeSigType::GenericMethodIndex(read_unsigned(bytes)?),
            _ => return Err(unsupported_blob()),
        })
    }
}

impl<'a> std::fmt::Display for TypeSigType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    fn new(file: &'a File, bytes: &mut &[u8]) -> ParseResult<TypeSig<'a>> {
        let array = read_expected(bytes, 0x1D)?;
        let modifiers = ModifierSig::vec(file, bytes)?;
        let sig_type = TypeSigType::new(file, bytes)?;
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

fn read_expected(bytes: &mut &[u8], expected: u32) -> ParseResult<bool> {
    let (element_type, bytes_read) = peek_unsigned(bytes)?;
    Ok(if element_type == expected {
        *bytes = seek(bytes, bytes_read);
        true
    } else {
        false
    })
}

fn read_string<'a>(bytes: &mut &'a [u8]) -> &'a str {
    let length = read_unsigned(bytes).unwrap();
    let (string_bytes, rest) = bytes.split_at(length as usize);
    *bytes = rest;
    std::str::from_utf8(string_bytes).unwrap()
}

fn read_i8(bytes: &mut &[u8]) -> i8 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i8>());
    *bytes = rest;
    i8::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u8(bytes: &mut &[u8]) -> u8 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u8>());
    *bytes = rest;
    u8::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i16(bytes: &mut &[u8]) -> i16 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i16>());
    *bytes = rest;
    i16::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u16(bytes: &mut &[u8]) -> u16 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u16>());
    *bytes = rest;
    u16::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i32(bytes: &mut &[u8]) -> i32 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i32>());
    *bytes = rest;
    i32::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u32(bytes: &mut &[u8]) -> u32 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u32>());
    *bytes = rest;
    u32::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_i64(bytes: &mut &[u8]) -> i64 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<i64>());
    *bytes = rest;
    i64::from_le_bytes(value_bytes.try_into().unwrap())
}

fn read_u64(bytes: &mut &[u8]) -> u64 {
    let (value_bytes, rest) = bytes.split_at(std::mem::size_of::<u64>());
    *bytes = rest;
    u64::from_le_bytes(value_bytes.try_into().unwrap())
}

fn seek(bytes: &[u8], bytes_read: usize) -> &[u8] {
    bytes.get(bytes_read..).unwrap()
}

fn peek_unsigned<'a>(bytes: &[u8]) -> ParseResult<(u32, usize)> {
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

fn read_unsigned<'a>(bytes: &mut &[u8]) -> ParseResult<u32> {
    let (value, bytes_read) = peek_unsigned(bytes)?;
    *bytes = seek(bytes, bytes_read);
    Ok(value)
}

#[derive(PartialEq)]
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
