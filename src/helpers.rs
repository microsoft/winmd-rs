use crate::error::*;

pub fn split_type_name(name: &str) -> ParseResult<(&str, &str)> {
    let index = name.rfind('.').ok_or_else(|| ParseError::InvalidTypeName)?;
    Ok((&name[0..index], &name[index + 1..]))
}
