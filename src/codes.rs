use crate::error::*;
use crate::file::*;
use crate::reader::*;
use crate::tables::*;
use winmd_macros::*;

#[type_code(2)]
pub enum TypeDefOrRef {
    TypeDef,
    TypeRef,
    TypeSpec,
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
    TypeSpec = 13,
    GenericParam = 19,
}

#[type_code(3)]
pub enum MemberRefParent {
    TypeDef,
    TypeRef,
    MethodDef = 3,
    TypeSpec,
}

#[type_code(2)]
pub enum HasConstant {
    Field,
    Param,
}

#[type_code(3)]
pub enum CustomAttributeType {
    MethodDef = 2,
    MemberRef,
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
            TypeDefOrRef::TypeDef(value) => write!(f, "{}.{}", value.namespace()?, value.name()?),
            TypeDefOrRef::TypeRef(value) => write!(f, "{}.{}", value.namespace()?, value.name()?),
            TypeDefOrRef::TypeSpec(_) => write!(f, "TypeSpec"),
        }
    }
}
