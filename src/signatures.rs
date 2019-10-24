use crate::codes::*;
use crate::error::*;
use crate::tables::*;
use std::io::Result;

struct ModifierSig<'a> {
    type_code: TypeDefOrRef<'a>,
}

pub struct MethodSig<'a> {
    return_sig: ReturnSig<'a>,
    params: std::vec::Vec<ParamSig<'a>>,
}

pub struct ParamSig<'a> {
    modifiers: std::vec::Vec<ModifierSig<'a>>,
    type_sig: Option<TypeSig>,
    by_ref: bool,
}

struct ReturnSig<'a> {
    modifiers: std::vec::Vec<ModifierSig<'a>>,
    type_sig: Option<TypeSig>,
    by_ref: bool,
}

struct TypeSig {}

impl<'a> MethodSig<'a> {
    pub(crate) fn new(method: &MethodDef<'_>) -> Result<MethodSig<'a>> {
        let mut bytes = method.row.blob(4)?;
        let (calling_convention, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);
        if calling_convention & 0x10 != 0 {
            let (_, bytes_read) = read_u32(&mut bytes)?;
            bytes = seek(bytes, bytes_read);
        }
        let (param_count, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);
        let return_sig = ReturnSig::new(&mut bytes)?;
        let mut params = std::vec::Vec::with_capacity(param_count as usize);
        for _ in 0..param_count {
            params.push(ParamSig::new(&mut bytes)?)
        }

        Err(invalid_blob())
    }
}

impl<'a> ModifierSig<'a> {
    fn new(bytes: &mut &[u8]) -> Result<ModifierSig<'a>> {
        Err(invalid_blob())
    }
}

impl<'a> ParamSig<'a> {
    fn new(bytes: &mut &[u8]) -> Result<ParamSig<'a>> {
        Err(invalid_blob())
    }
}

impl<'a> ReturnSig<'a> {
    fn new(bytes: &mut &[u8]) -> Result<ReturnSig<'a>> {
        let mut modifiers = std::vec::Vec::new();
        loop {
            let (element_type, _) = read_u32(bytes)?;
            if element_type != 32 && element_type != 31 {
                break;
            }
            modifiers.push(ModifierSig::new(bytes));
        }
        let (element_type, bytes_read) = read_u32(bytes)?;
        if element_type == 16 {}

        Err(invalid_blob())
    }
}

fn seek(bytes: &[u8], bytes_read: usize) -> &[u8] {
    bytes.get(bytes_read..).unwrap()
}

fn read_u32<'a>(bytes: &[u8]) -> Result<(u32, usize)> {
    if bytes.is_empty() {
        return Err(invalid_blob());
    }
    let bytes_read;
    let value = if bytes[0] & 0x80 == 0 {
        bytes_read = 1;
        bytes[0] as u32
    } else if bytes[0] & 0xC0 == 0x80 {
        if bytes.len() < 2 {
            return Err(invalid_blob());
        }
        bytes_read = 2;
        (((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32
    } else if bytes[0] & 0xE0 == 0xC0 {
        if bytes.len() < 4 {
            return Err(invalid_blob());
        }
        bytes_read = 4;
        (((bytes[0] & 0x1F) as u32) << 24) | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
    } else {
        return Err(invalid_blob());
    };

    Ok((value, bytes_read))
}

pub fn invalid_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported blob")
}
