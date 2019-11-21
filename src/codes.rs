use crate::file::*;
use crate::error::*;
use crate::tables::*;
use winmd_macros::*;

fn decode(bits: u32, code: u32) -> (u32, u32) {
    (code & ((1 << bits) - 1), (code >> bits) - 1)
}

fn encode(bits: u32, enumerator: u32, index: u32) -> u32 {
    ((index + 1) << bits) | enumerator
}

#[type_code(2)]
pub enum TypeDefOrRef {
    TypeDef,
    TypeRef,
    TypeSpec,
}

impl<'a> TypeDefOrRef<'a> {
    pub fn name(&'a self) -> ParseResult<&'a str> {
        match self {
            Self::TypeDef(value) => value.name(),
            Self::TypeRef(value) => value.name(),
            Self::TypeSpec(_) => panic!("Cannot call name() function on a TypeSpec"),
        }
    }

    pub fn namespace(&'a self) -> ParseResult<&'a str> {
        match self {
            Self::TypeDef(value) => value.namespace(),
            Self::TypeRef(value) => value.namespace(),
            Self::TypeSpec(_) => panic!("Cannot call namespace() function on a TypeSpec"),
        }
    }
}

impl<'a> std::fmt::Display for TypeDefOrRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDefOrRef::TypeDef(value) => write!(f, "{}.{}", value.namespace().unwrap(), value.name().unwrap()),
            TypeDefOrRef::TypeRef(value) => write!(f, "{}.{}", value.namespace().unwrap(), value.name().unwrap()),
            TypeDefOrRef::TypeSpec(_) => write!(f, "TypeSpec"),
        }
    }
}

#[type_code(5)]
pub enum HasCustomAttribute {
    MethodDef,
    Field,
    TypeRef,
    TypeDef,
    Param,
    InterfaceImpl,
    MemberRef,
    not_used,
    not_used,
    not_used,
    not_used,
    not_used,
    not_used,
    TypeSpec,
    not_used,
    not_used,
    not_used,
    not_used,
    not_used,
    GenericParam,
}

#[type_code(3)]
pub enum MemberRefParent {
    TypeDef,
    TypeRef,
    not_used,
    MethodDef,
    TypeSpec,
}

#[type_code(2)]
pub enum HasConstant {
    Field,
    Param,
}

#[type_code(3)]
pub enum CustomAttributeType {
    not_used,
    not_used,
    MethodDef,
    MemberRef,
}
