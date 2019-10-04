use crate::database::*;
use crate::tables::*;
use std::io::Result;

fn decode(bits: u32, code: u32) -> (u32, u32) {
    (code & ((1 << bits) - 1), (code >> bits) - 1)
}

pub enum TypeDefOrRef<'a> {
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    TypeSpec(TypeSpecRow<'a>),
}
impl<'a> TypeDefOrRef<'a> {
    pub fn decode(db: &'a Database, code: u32) -> TypeDefOrRef<'a> {
        let code = decode(2, code);
        match code.0 {
            0 => TypeDefOrRef::TypeDef(TypeDefRow { db: db, index: code.1 }),
            1 => TypeDefOrRef::TypeRef(TypeRefRow { db: db, index: code.1 }),
            _ => TypeDefOrRef::TypeSpec(TypeSpecRow { db: db, index: code.1 }),
        }
    }
    pub fn name(&self) -> Result<&'a str> {
        match self {
            TypeDefOrRef::TypeDef(row) => row.name(),
            TypeDefOrRef::TypeRef(row) => row.name(),
            TypeDefOrRef::TypeSpec(_) => panic!("Cannot call name() function on a TypeSpec"),
        }
    }
    pub fn namespace(&self) -> Result<&'a str> {
        match self {
            TypeDefOrRef::TypeDef(row) => row.namespace(),
            TypeDefOrRef::TypeRef(row) => row.namespace(),
            TypeDefOrRef::TypeSpec(_) => panic!("Cannot call namespace() function on a TypeSpec"),
        }
    }
}

pub enum HasCustomAttribute<'a> {
    MethodDef(MethodDefRow<'a>),
    Field(FieldRow<'a>),
    TypeRef(TypeRefRow<'a>),
    TypeDef(TypeDefRow<'a>),
    Param(ParamRow<'a>),
    InterfaceImpl(InterfaceImplRow<'a>),
    MemberRef(MemberRefRow<'a>),
    Module(ModuleRow<'a>),
    // Permission
    Property(PropertyRow<'a>),
    Event(EventRow<'a>),
    StandaloneSig(StandaloneSigRow<'a>),
    ModuleRef(ModuleRefRow<'a>),
    TypeSpec(TypeSpecRow<'a>),
    Assembly(AssemblyRow<'a>),
    AssemblyRef(AssemblyRefRow<'a>),
    File(FileRow<'a>),
    ExportedType(ExportedTypeRow<'a>),
    ManifestResource(ManifestResourceRow<'a>),
    GenericParam(GenericParamRow<'a>),
    GenericParamConstraint(GenericParamConstraintRow<'a>),
    MethodSpec(MethodSpecRow<'a>),
}
impl<'a> HasCustomAttribute<'a> {
    pub fn decode(db: &'a Database, code: u32) -> HasCustomAttribute<'a> {
        let code = decode(5, code);
        match code.0 {
            0 => HasCustomAttribute::MethodDef(MethodDefRow { db: db, index: code.1 }),
            1 => HasCustomAttribute::Field(FieldRow { db: db, index: code.1 }),
            2 => HasCustomAttribute::TypeRef(TypeRefRow { db: db, index: code.1 }),
            3 => HasCustomAttribute::TypeDef(TypeDefRow { db: db, index: code.1 }),
            4 => HasCustomAttribute::Param(ParamRow { db: db, index: code.1 }),
            5 => HasCustomAttribute::InterfaceImpl(InterfaceImplRow { db: db, index: code.1 }),
            6 => HasCustomAttribute::MemberRef(MemberRefRow { db: db, index: code.1 }),
            7 => HasCustomAttribute::Module(ModuleRow { db: db, index: code.1 }),
            // Permission
            9 => HasCustomAttribute::Property(PropertyRow { db: db, index: code.1 }),
            10 => HasCustomAttribute::Event(EventRow { db: db, index: code.1 }),
            11 => HasCustomAttribute::StandaloneSig(StandaloneSigRow { db: db, index: code.1 }),
            12 => HasCustomAttribute::ModuleRef(ModuleRefRow { db: db, index: code.1 }),
            13 => HasCustomAttribute::TypeSpec(TypeSpecRow { db: db, index: code.1 }),
            14 => HasCustomAttribute::Assembly(AssemblyRow { db: db, index: code.1 }),
            15 => HasCustomAttribute::AssemblyRef(AssemblyRefRow { db: db, index: code.1 }),
            16 => HasCustomAttribute::File(FileRow { db: db, index: code.1 }),
            17 => HasCustomAttribute::ExportedType(ExportedTypeRow { db: db, index: code.1 }),
            18 => HasCustomAttribute::ManifestResource(ManifestResourceRow { db: db, index: code.1 }),
            19 => HasCustomAttribute::GenericParam(GenericParamRow { db: db, index: code.1 }),
            20 => HasCustomAttribute::GenericParamConstraint(GenericParamConstraintRow { db: db, index: code.1 }),
            21 => HasCustomAttribute::MethodSpec(MethodSpecRow { db: db, index: code.1 }),
            _ => panic!("Invalid HasCustomAttribute code"),
        }
    }
}
