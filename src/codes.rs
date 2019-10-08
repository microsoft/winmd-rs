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
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    TypeSpec(TypeSpecRow<'a>),
}
impl<'a> TypeDefOrRef<'a> {
    pub fn decode(db: &'a Database, code: u32) -> TypeDefOrRef<'a> {
        let code = decode(2, code);
        match code.0 {
            0 => TypeDefOrRef::TypeDef(TypeDef::new(db, code.1)),
            1 => TypeDefOrRef::TypeRef(TypeRef::new(db, code.1)),
            2 => TypeDefOrRef::TypeSpec(TypeSpec::new(db, code.1)),
            _ => panic!("Invalid TypeDefOrRef code"),
        }
    }
    pub fn encode(&self) -> u32 {
        match self {
            TypeDefOrRef::TypeDef(row) => encode(2, 0, row.first),
            TypeDefOrRef::TypeRef(row) => encode(2, 1, row.first),
            TypeDefOrRef::TypeSpec(row) => encode(2, 2, row.first),
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
            0 => HasCustomAttribute::MethodDef(MethodDef::new(db, code.1)),
            1 => HasCustomAttribute::Field(Field::new(db, code.1)),
            2 => HasCustomAttribute::TypeRef(TypeRef::new(db, code.1)),
            3 => HasCustomAttribute::TypeDef(TypeDef::new(db, code.1)),
            4 => HasCustomAttribute::Param(Param::new(db, code.1)),
            5 => HasCustomAttribute::InterfaceImpl(InterfaceImpl::new(db, code.1)),
            6 => HasCustomAttribute::MemberRef(MemberRef::new(db, code.1)),
            7 => HasCustomAttribute::Module(Module::new(db, code.1)),
            // Permission
            9 => HasCustomAttribute::Property(Property::new(db, code.1)),
            10 => HasCustomAttribute::Event(Event::new(db, code.1)),
            11 => HasCustomAttribute::StandaloneSig(StandaloneSig::new(db, code.1)),
            12 => HasCustomAttribute::ModuleRef(ModuleRef::new(db, code.1)),
            13 => HasCustomAttribute::TypeSpec(TypeSpec::new(db, code.1)),
            14 => HasCustomAttribute::Assembly(Assembly::new(db, code.1)),
            15 => HasCustomAttribute::AssemblyRef(AssemblyRef::new(db, code.1)),
            16 => HasCustomAttribute::File(File::new(db, code.1)),
            17 => HasCustomAttribute::ExportedType(ExportedType::new(db, code.1)),
            18 => HasCustomAttribute::ManifestResource(ManifestResource::new(db, code.1)),
            19 => HasCustomAttribute::GenericParam(GenericParam::new(db, code.1)),
            20 => HasCustomAttribute::GenericParamConstraint(GenericParamConstraint::new(db, code.1)),
            21 => HasCustomAttribute::MethodSpec(MethodSpec::new(db, code.1)),
            _ => panic!("Invalid HasCustomAttribute code"),
        }
    }
    pub fn encode(&self) -> u32 {
        match self {
            HasCustomAttribute::MethodDef(row) => encode(5, 0, row.first),
            HasCustomAttribute::Field(row) => encode(5, 1, row.first),
            HasCustomAttribute::TypeRef(row) => encode(5, 2, row.first),
            HasCustomAttribute::TypeDef(row) => encode(5, 3, row.first),
            HasCustomAttribute::Param(row) => encode(5, 4, row.first),
            HasCustomAttribute::InterfaceImpl(row) => encode(5, 5, row.first),
            HasCustomAttribute::MemberRef(row) => encode(5, 6, row.first),
            HasCustomAttribute::Module(row) => encode(5, 7, row.first),
            // Permission
            HasCustomAttribute::Property(row) => encode(5, 9, row.first),
            HasCustomAttribute::Event(row) => encode(5, 10, row.first),
            HasCustomAttribute::StandaloneSig(row) => encode(5, 11, row.first),
            HasCustomAttribute::ModuleRef(row) => encode(5, 12, row.first),
            HasCustomAttribute::TypeSpec(row) => encode(5, 13, row.first),
            HasCustomAttribute::Assembly(row) => encode(5, 14, row.first),
            HasCustomAttribute::AssemblyRef(row) => encode(5, 15, row.first),
            HasCustomAttribute::File(row) => encode(5, 16, row.first),
            HasCustomAttribute::ExportedType(row) => encode(5, 17, row.first),
            HasCustomAttribute::ManifestResource(row) => encode(5, 18, row.first),
            HasCustomAttribute::GenericParam(row) => encode(5, 19, row.first),
            HasCustomAttribute::GenericParamConstraint(row) => encode(5, 20, row.first),
            HasCustomAttribute::MethodSpec(row) => encode(5, 21, row.first),
        }
    }
    pub fn database(&self) -> &Database {
        match self {
            HasCustomAttribute::MethodDef(row) => row.db,
            HasCustomAttribute::Field(row) => row.db,
            HasCustomAttribute::TypeRef(row) => row.db,
            HasCustomAttribute::TypeDef(row) => row.db,
            HasCustomAttribute::Param(row) => row.db,
            HasCustomAttribute::InterfaceImpl(row) => row.db,
            HasCustomAttribute::MemberRef(row) => row.db,
            HasCustomAttribute::Module(row) => row.db,
            // Permission
            HasCustomAttribute::Property(row) => row.db,
            HasCustomAttribute::Event(row) => row.db,
            HasCustomAttribute::StandaloneSig(row) => row.db,
            HasCustomAttribute::ModuleRef(row) => row.db,
            HasCustomAttribute::TypeSpec(row) => row.db,
            HasCustomAttribute::Assembly(row) => row.db,
            HasCustomAttribute::AssemblyRef(row) => row.db,
            HasCustomAttribute::File(row) => row.db,
            HasCustomAttribute::ExportedType(row) => row.db,
            HasCustomAttribute::ManifestResource(row) => row.db,
            HasCustomAttribute::GenericParam(row) => row.db,
            HasCustomAttribute::GenericParamConstraint(row) => row.db,
            HasCustomAttribute::MethodSpec(row) => row.db,
        }
    }
}

pub enum CustomAttributeType<'a> {
    MethodDef(MethodDefRow<'a>),
    MemberRef(MemberRefRow<'a>),
}
impl<'a> CustomAttributeType<'a> {
    pub fn decode(db: &'a Database, code: u32) -> CustomAttributeType<'a> {
        let code = decode(3, code);
        match code.0 {
            2 => CustomAttributeType::MethodDef(MethodDef::new(db, code.1)),
            3 => CustomAttributeType::MemberRef(MemberRef::new(db, code.1)),
            _ => panic!("Invalid CustomAttributeType code"),
        }
    }
}

pub enum MemberRefParent<'a> {
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    ModuleRef(ModuleRefRow<'a>),
    MethodDef(MethodDefRow<'a>),
    TypeSpec(TypeSpecRow<'a>),
}
impl<'a> MemberRefParent<'a> {
    pub fn decode(db: &'a Database, code: u32) -> MemberRefParent<'a> {
        let code = decode(3, code);
        match code.0 {
            0 => MemberRefParent::TypeDef(TypeDef::new(db, code.1)),
            1 => MemberRefParent::TypeRef(TypeRef::new(db, code.1)),
            2 => MemberRefParent::ModuleRef(ModuleRef::new(db, code.1)),
            3 => MemberRefParent::MethodDef(MethodDef::new(db, code.1)),
            4 => MemberRefParent::TypeSpec(TypeSpec::new(db, code.1)),
            _ => panic!("Invalid MemberRefParent code"),
        }
    }
}
