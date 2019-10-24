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
        let blob = method.row.blob(4);

        Err(invalid_blob())
    }
}

fn read_u32(bytes: &[u8], bytes_read: &mut usize) -> Result<u32> {
    if bytes.is_empty() {
        return Err(invalid_blob());
    }
    Ok(if bytes[0] & 0x80 == 0 {
        *bytes_read = 1;
        bytes[0] as u32
    } else if bytes[0] & 0xC0 == 0x80 {
        if bytes.len() < 2 {
            return Err(invalid_blob());
        }
        *bytes_read = 2;
        (((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32
    } else if bytes[0] & 0xE0 == 0xC0 {
        if bytes.len() < 4 {
            return Err(invalid_blob());
        }
        *bytes_read = 4;
        (((bytes[0] & 0x1F) as u32) << 24) | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
    } else {
        return Err(invalid_blob());
    })
}

pub fn invalid_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid blob")
}
