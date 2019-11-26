use crate::error::*;

pub fn split_type_name(name: &str) -> ParseResult<(&str, &str)> {
    let index = name.rfind('.').ok_or_else(|| ParseError::InvalidTypeName)?;
    Ok((name.get(0..index).unwrap(), name.get(index + 1..).unwrap()))
}
