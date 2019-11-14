use crate::database::*;
use crate::error::*;
use crate::tables::*;
use winmd_macros::*;

#[type_encoding(1)]
enum TypeDefOrRef2 {
    TypeDef,
    TypeRef,
    TypeSpec,
}

//fn use_gen(code: &TypeDefOrRef2) {}

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
    pub(crate) fn decode(db: &'a Database, code: u32) -> ParseResult<Self> {
        let code = decode(2, code);
        Ok(match code.0 {
            0 => Self::TypeDef(db.type_def().row(code.1)),
            1 => Self::TypeRef(db.type_ref().row(code.1)),
            2 => Self::TypeSpec(db.type_spec().row(code.1)),
            _ => return Err(ParseError::InvalidData("Invalid TypeDefOrRef code")),
        })
    }

    pub fn encode(&self) -> u32 {
        match &self {
            Self::TypeDef(value) => encode(2, 0, value.row.index),
            Self::TypeRef(value) => encode(2, 1, value.row.index),
            Self::TypeSpec(value) => encode(2, 2, value.row.index),
        }
    }

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

pub enum HasCustomAttribute<'a> {
    MethodDef(MethodDef<'a>),
    Field(Field<'a>),
    TypeRef(TypeRef<'a>),
    TypeDef(TypeDef<'a>),
    Param(Param<'a>),
    InterfaceImpl(InterfaceImpl<'a>),
    MemberRef(MemberRef<'a>),
    Module(Module<'a>),
    Permission(Permission<'a>),
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
    pub(crate) fn decode(db: &'a Database, code: u32) -> ParseResult<Self> {
        let code = decode(5, code);
        Ok(match code.0 {
            0 => Self::MethodDef(db.method_def().row(code.1)),
            1 => Self::Field(db.field().row(code.1)),
            2 => Self::TypeRef(db.type_ref().row(code.1)),
            3 => Self::TypeDef(db.type_def().row(code.1)),
            4 => Self::Param(db.param().row(code.1)),
            5 => Self::InterfaceImpl(db.interface_impl().row(code.1)),
            6 => Self::MemberRef(db.member_ref().row(code.1)),
            7 => Self::Module(db.module().row(code.1)),
            8 => Self::Permission(db.module().row(code.1)),
            9 => Self::Property(db.property().row(code.1)),
            10 => Self::Event(db.event().row(code.1)),
            11 => Self::StandaloneSig(db.standalone_sig().row(code.1)),
            12 => Self::ModuleRef(db.module_ref().row(code.1)),
            13 => Self::TypeSpec(db.type_spec().row(code.1)),
            14 => Self::Assembly(db.assembly().row(code.1)),
            15 => Self::AssemblyRef(db.assembly_ref().row(code.1)),
            16 => Self::File(db.file().row(code.1)),
            17 => Self::ExportedType(db.exported_type().row(code.1)),
            18 => Self::ManifestResource(db.manifest_resource().row(code.1)),
            19 => Self::GenericParam(db.generic_param().row(code.1)),
            20 => Self::GenericParamConstraint(db.generic_param_constraint().row(code.1)),
            21 => Self::MethodSpec(db.method_spec().row(code.1)),
            _ => return Err(ParseError::InvalidData("Invalid HasCustomAttribute code")),
        })
    }

    pub fn encode(&self) -> u32 {
        match &self {
            Self::MethodDef(value) => encode(5, 0, value.row.index),
            Self::Field(value) => encode(5, 1, value.row.index),
            Self::TypeRef(value) => encode(5, 2, value.row.index),
            Self::TypeDef(value) => encode(5, 3, value.row.index),
            Self::Param(value) => encode(5, 4, value.row.index),
            Self::InterfaceImpl(value) => encode(5, 5, value.row.index),
            Self::MemberRef(value) => encode(5, 6, value.row.index),
            Self::Module(value) => encode(5, 7, value.row.index),
            Self::Permission(value) => encode(5, 8, value.row.index),
            Self::Property(value) => encode(5, 9, value.row.index),
            Self::Event(value) => encode(5, 10, value.row.index),
            Self::StandaloneSig(value) => encode(5, 11, value.row.index),
            Self::ModuleRef(value) => encode(5, 12, value.row.index),
            Self::TypeSpec(value) => encode(5, 13, value.row.index),
            Self::Assembly(value) => encode(5, 14, value.row.index),
            Self::AssemblyRef(value) => encode(5, 15, value.row.index),
            Self::File(value) => encode(5, 16, value.row.index),
            Self::ExportedType(value) => encode(5, 17, value.row.index),
            Self::ManifestResource(value) => encode(5, 18, value.row.index),
            Self::GenericParam(value) => encode(5, 19, value.row.index),
            Self::GenericParamConstraint(value) => encode(5, 20, value.row.index),
            Self::MethodSpec(value) => encode(5, 21, value.row.index),
        }
    }
}

pub enum CustomAttributeType<'a> {
    MethodDef(MethodDef<'a>),
    MemberRef(MemberRef<'a>),
}

impl<'a> CustomAttributeType<'a> {
    pub(crate) fn decode(db: &'a Database, code: u32) -> ParseResult<Self> {
        let code = decode(3, code);
        Ok(match code.0 {
            2 => Self::MethodDef(db.method_def().row(code.1)),
            3 => Self::MemberRef(db.member_ref().row(code.1)),
            _ => return Err(ParseError::InvalidData("Invalid CustomAttributeType code")),
        })
    }
}

pub enum MemberRefParent<'a> {
    TypeDef(TypeDef<'a>),
    TypeRef(TypeRef<'a>),
    ModuleRef(ModuleRef<'a>),
    MethodDef(MethodDef<'a>),
    TypeSpec(TypeSpec<'a>),
}

impl<'a> MemberRefParent<'a> {
    pub(crate) fn decode(db: &'a Database, code: u32) -> ParseResult<Self> {
        let code = decode(3, code);
        Ok(match code.0 {
            0 => Self::TypeDef(db.type_def().row(code.1)),
            1 => Self::TypeRef(db.type_ref().row(code.1)),
            2 => Self::ModuleRef(db.module_ref().row(code.1)),
            3 => Self::MethodDef(db.method_def().row(code.1)),
            4 => Self::TypeSpec(db.type_spec().row(code.1)),
            _ => return Err(ParseError::InvalidData("Invalid MemberRefParent code")),
        })
    }
}

pub enum HasConstant<'a> {
    Field(Field<'a>),
    Param(Param<'a>),
    Property(Property<'a>),
}

impl<'a> HasConstant<'a> {
    #![allow(dead_code)]
    pub(crate) fn decode(db: &'a Database, code: u32) -> ParseResult<Self> {
        let code = decode(2, code);
        Ok(match code.0 {
            0 => Self::Field(db.field().row(code.1)),
            1 => Self::Param(db.param().row(code.1)),
            2 => Self::Property(db.property().row(code.1)),
            _ => return Err(ParseError::InvalidData("Invalid HasConstant code")),
        })
    }

    pub fn encode(&self) -> u32 {
        match &self {
            Self::Field(value) => encode(2, 0, value.row.index),
            Self::Param(value) => encode(2, 1, value.row.index),
            Self::Property(value) => encode(2, 2, value.row.index),
        }
    }
}
