use crate::database::*;
use crate::tables::*;
use std::io::Result;

fn decode(bits: u32, code: u32) -> (u32, u32) {
    (code & ((1 << bits) - 1), (code >> bits) - 1)
}

pub enum TypeDefOrRef<'a> {
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    // TODO: TypeSpec,
}

use TypeDefOrRef::*;

impl<'a> TypeDefOrRef<'a> {
    pub fn decode(db: &'a Database, code: u32) -> TypeDefOrRef<'a> {
        let code = decode(2, code);
        match code.0 {
            0 => TypeDef(TypeDefRow { db: db, index: code.1 }),
            _ => TypeRef(TypeRefRow { db: db, index: code.1 }),
        }
    }
    pub fn name(&self) -> Result<&'a str> {
        match self {
            TypeDef(row) => row.name(),
            TypeRef(row) => row.name(),
            _ => panic!("Cannot call name() function on a TypeSpec"),
        }
    }
    pub fn namespace(&self) -> Result<&'a str> {
        match self {
            TypeDef(row) => row.namespace(),
            TypeRef(row) => row.namespace(),
            _ => panic!("Cannot call namespace() function on a TypeSpec"),
        }
    }
}
