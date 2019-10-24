use crate::error::*;
use crate::tables::*;
use std::io::Result;

pub struct MethodSig {
    // return_type : return_type
// params : param
}

pub struct ParamSig{

}

struct ReturnSig {}

struct TypeSignature {}

impl MethodSig {
    pub(crate) fn new(method: &MethodDef<'_>) -> Result<MethodSig> {
        let mut blob = method.row.blob(4)?;
        let mut calling_convention: u32 = 0;
        blob = read_u32(blob, &mut calling_convention)?;
        let mut generic_params: u32 = 0;
        if calling_convention & 0x10 != 0 {
            blob = read_u32(blob, &mut calling_convention)?;
        }
        let mut params: u32 = 0;
        blob = read_u32(blob, &mut params)?;

        Err(invalid_blob())
    }
}

impl ParamSig{
    fn new(bytes: &[u8]) -> Result<ParamSig>
    {
Err(invalid_blob())
    }
}

impl ReturnSig{
    fn new(bytes: &[u8]) -> Result<ReturnSig>
    {
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
    Ok(bytes.get(bytes_read..).unwrap())
}

pub fn invalid_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid blob")
}
