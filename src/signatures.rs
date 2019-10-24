use crate::error::*;
use crate::tables::*;
use std::io::Result;

pub struct MethodSignature {
// return_type : return_type
// params : param
}

struct ReturnTypeSignature {}

struct TypeSignature {}

impl MethodSignature {
    pub(crate) fn new(method: &MethodDef<'_>) -> Result<MethodSignature> {
        let mut blob = method.row.blob(4)?;
        let mut calling_convention: u32 = 0;
        blob = read_u32(blob, &mut calling_convention)?;

        Err(invalid_blob())
    }
}

fn read_u32<'a>(bytes: &'a [u8], value: &mut u32) -> Result<&'a [u8]> {
    if bytes.is_empty() {
        return Err(invalid_blob());
    }
    let bytes_read;
    *value = if bytes[0] & 0x80 == 0 {
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

    // TODO: can avoid the unwrap if we fold that into the if/else chain.
    Ok(bytes.get(bytes_read..).unwrap())
}

pub fn invalid_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid blob")
}
