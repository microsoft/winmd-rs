#![allow(dead_code)]

use crate::database::*;
use crate::tables::*;
use std::io::Result;

fn decode(bits: u32, code: u32) -> (u32, u32) {
    (code & ((1 << bits) - 1), (code >> bits) - 1)
}
fn encode(bits: u32, enumerator: u32, index: u32) -> u32 {
    ((index + 1) << bits) | enumerator
}

pub enum TypeDefOrRef<'a> {
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    TypeSpec(TypeSpec<'a>),
}
impl<'a> TypeDefOrRef<'a> {
    pub(crate) fn decode(db: &'a Database, code: u32) -> Self {
        let code = decode(2, code);
        match code.0 {
            0 => Self::TypeDef(db.type_def().row(code.1)),
            1 => Self::TypeRef(db.type_ref().row(code.1)),
            2 => Self::TypeSpec(db.type_spec().row(code.1)),
            _ => panic!("Invalid TypeDefOrRef code"),
        }
    }
    pub fn encode(&self) -> u32 {
        match &self {
            Self::TypeDef(row) => encode(2, 0, row.data.index),
            Self::TypeRef(row) => encode(2, 1, row.data.index),
            Self::TypeSpec(row) => encode(2, 2, row.data.index),
        }
    }
    pub fn name(&'a self) -> Result<&'a str> {
        match self {
            Self::TypeDef(row) => row.name(),
            Self::TypeRef(row) => row.name(),
            Self::TypeSpec(_) => panic!("Cannot call name() function on a TypeSpec"),
        }
    }
    pub fn namespace(&'a self) -> Result<&'a str> {
        match self {
            Self::TypeDef(row) => row.namespace(),
            Self::TypeRef(row) => row.namespace(),
            Self::TypeSpec(_) => panic!("Cannot call namespace() function on a TypeSpec"),
        }
    }
}

pub enum HasCustomAttribute<'a> {
    MethodDef(MethodDef<'a>),
    Field(Field<'a>),
    TypeRef(TypeRef<'a>),
    TypeDef(TypeDef<'a>),
    Param(Param<'a>),
    InterfaceImpl(InterfaceImpl<'a>),
    MemberRef(MemberRef<'a>),
    Module(Module<'a>),
    // Permission
    Property(Property<'a>),
    Event(Event<'a>),
    StandaloneSig(StandaloneSig<'a>),
    ModuleRef(ModuleRef<'a>),
    TypeSpec(TypeSpec<'a>),
    Assembly(Assembly<'a>),
    AssemblyRef(AssemblyRef<'a>),
    File(File<'a>),
    ExportedType(ExportedType<'a>),
    ManifestResource(ManifestResource<'a>),
    GenericParam(GenericParam<'a>),
    GenericParamConstraint(GenericParamConstraint<'a>),
    MethodSpec(MethodSpec<'a>),
}
impl<'a> HasCustomAttribute<'a> {
    pub(crate) fn decode(db: &'a Database, code: u32) -> Self {
        let code = decode(5, code);
        match code.0 {
            0 => Self::MethodDef(db.method_def().row(code.1)),
            // 1 => Self::Field(Field::new(db, code.1)),
            // 2 => Self::TypeRef(TypeRef::new(db, code.1)),
            // 3 => Self::TypeDef2(TypeDef2::new(db, code.1)),
            // 4 => Self::Param(Param::new(db, code.1)),
            // 5 => Self::InterfaceImpl(InterfaceImpl::new(db, code.1)),
            // 6 => Self::MemberRef(MemberRef::new(db, code.1)),
            // 7 => Self::Module(Module::new(db, code.1)),
            // // Permission
            // 9 => Self::Property(Property::new(db, code.1)),
            // 10 => Self::Event(Event::new(db, code.1)),
            // 11 => Self::StandaloneSig(StandaloneSig::new(db, code.1)),
            // 12 => Self::ModuleRef(ModuleRef::new(db, code.1)),
            // 13 => Self::TypeSpec(TypeSpec::new(db, code.1)),
            // 14 => Self::Assembly(Assembly::new(db, code.1)),
            // 15 => Self::AssemblyRef(AssemblyRef::new(db, code.1)),
            // 16 => Self::File(File::new(db, code.1)),
            // 17 => Self::ExportedType(ExportedType::new(db, code.1)),
            // 18 => Self::ManifestResource(ManifestResource::new(db, code.1)),
            // 19 => Self::GenericParam(GenericParam::new(db, code.1)),
            // 20 => Self::GenericParamConstraint(GenericParamConstraint::new(db, code.1)),
            // 21 => Self::MethodSpec(MethodSpec::new(db, code.1)),
            _ => panic!("Invalid HasCustomAttribute code"),
        }
    }
    //     pub fn encode(&self) -> u32 {
    //         match self {
    //             Self::MethodDef(row) => encode(5, 0, row.first),
    //             Self::Field(row) => encode(5, 1, row.first),
    //             Self::TypeRef(row) => encode(5, 2, row.first),
    //             Self::TypeDef2(row) => encode(5, 3, row.first),
    //             Self::Param(row) => encode(5, 4, row.first),
    //             Self::InterfaceImpl(row) => encode(5, 5, row.first),
    //             Self::MemberRef(row) => encode(5, 6, row.first),
    //             Self::Module(row) => encode(5, 7, row.first),
    //             // Permission
    //             Self::Property(row) => encode(5, 9, row.first),
    //             Self::Event(row) => encode(5, 10, row.first),
    //             Self::StandaloneSig(row) => encode(5, 11, row.first),
    //             Self::ModuleRef(row) => encode(5, 12, row.first),
    //             Self::TypeSpec(row) => encode(5, 13, row.first),
    //             Self::Assembly(row) => encode(5, 14, row.first),
    //             Self::AssemblyRef(row) => encode(5, 15, row.first),
    //             Self::File(row) => encode(5, 16, row.first),
    //             Self::ExportedType(row) => encode(5, 17, row.first),
    //             Self::ManifestResource(row) => encode(5, 18, row.first),
    //             Self::GenericParam(row) => encode(5, 19, row.first),
    //             Self::GenericParamConstraint(row) => encode(5, 20, row.first),
    //             Self::MethodSpec(row) => encode(5, 21, row.first),
    //         }
    //     }
    //     pub(crate) fn database(&self) -> &Database {
    //         match self {
    //             Self::MethodDef(row) => row.db,
    //             Self::Field(row) => row.db,
    //             Self::TypeRef(row) => row.db,
    //             Self::TypeDef2(row) => row.db,
    //             Self::Param(row) => row.db,
    //             Self::InterfaceImpl(row) => row.db,
    //             Self::MemberRef(row) => row.db,
    //             Self::Module(row) => row.db,
    //             // Permission
    //             Self::Property(row) => row.db,
    //             Self::Event(row) => row.db,
    //             Self::StandaloneSig(row) => row.db,
    //             Self::ModuleRef(row) => row.db,
    //             Self::TypeSpec(row) => row.db,
    //             Self::Assembly(row) => row.db,
    //             Self::AssemblyRef(row) => row.db,
    //             Self::File(row) => row.db,
    //             Self::ExportedType(row) => row.db,
    //             Self::ManifestResource(row) => row.db,
    //             Self::GenericParam(row) => row.db,
    //             Self::GenericParamConstraint(row) => row.db,
    //             Self::MethodSpec(row) => row.db,
    //         }
    //     }
}

pub enum CustomAttributeType<'a> {
    MethodDef(MethodDef<'a>),
    MemberRef(MemberRef<'a>),
}
// impl<'a> CustomAttributeType<'a> {
//     pub(crate) fn decode(db: &'a Database, code: u32) -> Self {
//         let code = decode(3, code);
//         match code.0 {
//             2 => Self::MethodDef(MethodDef::new(db, code.1)),
//             3 => Self::MemberRef(MemberRef::new(db, code.1)),
//             _ => panic!("Invalid CustomAttributeType code"),
//         }
//     }
// }

pub enum MemberRefParent<'a> {
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    ModuleRef(ModuleRef<'a>),
    MethodDef(MethodDef<'a>),
    TypeSpec(TypeSpec<'a>),
}
// impl<'a> MemberRefParent<'a> {
//     pub(crate) fn decode(db: &'a Database, code: u32) -> Self {
//         let code = decode(3, code);
//         match code.0 {
//             0 => Self::TypeDef2(TypeDef2::new(db, code.1)),
//             1 => Self::TypeRef(TypeRef::new(db, code.1)),
//             2 => Self::ModuleRef(ModuleRef::new(db, code.1)),
//             3 => Self::MethodDef(MethodDef::new(db, code.1)),
//             4 => Self::TypeSpec(TypeSpec::new(db, code.1)),
//             _ => panic!("Invalid MemberRefParent code"),
//         }
//     }
// }
