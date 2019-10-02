use crate::database::*;
use crate::tables::*;

pub trait CodedIndex {
    // fn bits(&self) -> u32;
    fn decode<'a>(db: &'a Database, index: u32) -> TypeDefOrRef<'a>;
}

pub enum TypeDefOrRef<'a> {
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    // TypeSpec,
}

impl<'a> TypeDefOrRef<'a> {
    pub fn name(&self) -> std::io::Result<&'a str> {
        match self {
            TypeDefOrRef::TypeDef(row) => row.name(),
            TypeDefOrRef::TypeRef(row) => row.name(),
            _ => panic!(),
        }
    }
        pub fn namespace(&self) -> std::io::Result<&'a str> {
        match self {
            TypeDefOrRef::TypeDef(row) => row.namespace(),
            TypeDefOrRef::TypeRef(row) => row.namespace(),
            _ => panic!(),
        }
    }
}

impl CodedIndex for TypeDefOrRef<'_> {
    fn decode<'a>(db: &'a Database, code: u32) -> TypeDefOrRef<'a> {
        const BITS: u32 = 2;
        let index = (code >> BITS) - 1;

        match code & ((1 << BITS) - 1) {
            0 => TypeDefOrRef::TypeDef(TypeDefRow { db: db, index: index }),
            _ => TypeDefOrRef::TypeRef(TypeRefRow { db: db, index: index }),
        }
    }
}
