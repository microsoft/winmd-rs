use crate::codes::*;
use crate::database::*;
use crate::error::*;
use crate::tables::*;
use std::io::Result;

// TODO: what about using std::io::Read?

pub struct GenericSig<'a> {
    sig_type: TypeDefOrRef<'a>,
    args: std::vec::Vec<TypeSig<'a>>,
}

pub struct ModifierSig<'a> {
    type_code: TypeDefOrRef<'a>,
}
impl<'a> ModifierSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<ModifierSig<'a>> {
        let (need, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);
        let (code, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);
        let type_code = TypeDefOrRef::decode(db, code);
        Ok(ModifierSig { type_code })
    }
    fn vec(db: &'a Database, bytes: &mut &[u8]) -> Result<std::vec::Vec<ModifierSig<'a>>> {
        let mut modifiers = std::vec::Vec::new();
        loop {
            let (element_type, _) = read_u32(bytes)?;
            if element_type != 32 && element_type != 31 {
                break;
            }
            modifiers.push(ModifierSig::new(db, bytes)?);
        }
        Ok(modifiers)
    }
}

// TODO: do as the cppwinrt's method_signature does and stick the params in a vector of pairs along with their names
pub struct MethodSig<'a> {
    pub return_sig: Option<TypeSig<'a>>,
    pub params: std::vec::Vec<ParamSig<'a>>,
}
impl<'a> MethodSig<'a> {
    pub(crate) fn new(method: &MethodDef<'a>) -> Result<MethodSig<'a>> {
        let mut bytes = method.row.blob(4)?;
        let (calling_convention, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);
        if calling_convention & 0x10 != 0 {
            let (_, bytes_read) = read_u32(&mut bytes)?;
            bytes = seek(bytes, bytes_read);
        }
        let (param_count, bytes_read) = read_u32(&mut bytes)?;
        bytes = seek(bytes, bytes_read);

        let return_sig;
        {
            let modifiers = ModifierSig::vec(method.row.table.db, &mut bytes)?;
            let by_ref = read_expected(&mut bytes, 0x10)?;
            if read_expected(&mut bytes, 0x01)? {
                return_sig = None;
            } else {
                return_sig = Some(TypeSig::new(method.row.table.db, &mut bytes)?);
            }
        }

        let mut params = std::vec::Vec::with_capacity(param_count as usize);
        for _ in 0..param_count {
            params.push(ParamSig::new(method.row.table.db, &mut bytes)?);
        }
        Ok(MethodSig { return_sig, params })
    }
}

pub struct ParamSig<'a> {
    modifiers: std::vec::Vec<ModifierSig<'a>>,
    by_ref: bool,
    pub type_sig: TypeSig<'a>,
}
impl<'a> ParamSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<ParamSig<'a>> {
        let modifiers = ModifierSig::vec(db, bytes)?;
        let by_ref = read_expected(bytes, 0x10)?;
        let type_sig = TypeSig::new(db, bytes)?;
        Ok(ParamSig { modifiers, by_ref, type_sig })
    }
}

pub enum TypeSigType<'a> {
    ElementType(ElementType),
    TypeDefOrRef(TypeDefOrRef<'a>),
    GenericSig(GenericSig<'a>),
    GenericTypeIndex(u32),
    GenericMethodIndex(u32),
}
impl<'a> TypeSigType<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<TypeSigType<'a>> {
        let (element_type, bytes_read) = read_u32(bytes)?;
        *bytes = seek(bytes, bytes_read);

        Ok(match element_type {
            0x02 => TypeSigType::ElementType(ElementType::Boolean),
            0x03 => TypeSigType::ElementType(ElementType::Char),
            0x04 => TypeSigType::ElementType(ElementType::I1),
            0x05 => TypeSigType::ElementType(ElementType::U1),
            0x06 => TypeSigType::ElementType(ElementType::I2),
            0x07 => TypeSigType::ElementType(ElementType::U2),
            0x08 => TypeSigType::ElementType(ElementType::I4),
            0x09 => TypeSigType::ElementType(ElementType::U4),
            0x0A => TypeSigType::ElementType(ElementType::I8),
            0x0B => TypeSigType::ElementType(ElementType::U8),
            0x0C => TypeSigType::ElementType(ElementType::R4),
            0x0D => TypeSigType::ElementType(ElementType::R8),
            0x0E => TypeSigType::ElementType(ElementType::String),
            0x1C => TypeSigType::ElementType(ElementType::Object),
            0x18 => TypeSigType::ElementType(ElementType::I),
            0x19 => TypeSigType::ElementType(ElementType::U),
            0x11 | 0x12 => {
                let (code, bytes_read) = read_u32(bytes)?;
                *bytes = seek(bytes, bytes_read);
                TypeSigType::TypeDefOrRef(TypeDefOrRef::decode(db, code))
            }

            _ => return Err(unsupported_blob()),
        })
    }
}
impl<'a> std::fmt::Display for TypeSigType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSigType::ElementType(value) => write!(f, "{}", value),
            TypeSigType::TypeDefOrRef(value) => write!(f, "{}", value),
            TypeSigType::GenericSig(value) => write!(f, "{}", "GenericSig"),
            TypeSigType::GenericTypeIndex(value) => write!(f, "{}", value),
            TypeSigType::GenericMethodIndex(value) => write!(f, "{}", value),
        }
    }
}

pub struct TypeSig<'a> {
    array: bool,
    modifiers: std::vec::Vec<ModifierSig<'a>>,
    sig_type: TypeSigType<'a>,
}
impl<'a> TypeSig<'a> {
    fn new(db: &'a Database, bytes: &mut &[u8]) -> Result<TypeSig<'a>> {
        let array = read_expected(bytes, 0x1D)?;
        let modifiers = ModifierSig::vec(db, bytes)?;
        let sig_type = TypeSigType::new(db, bytes)?;
        Ok(TypeSig { array, modifiers, sig_type })
    }
}
impl<'a> std::fmt::Display for TypeSig<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sig_type)
    }
}

fn read_expected(bytes: &mut &[u8], expected: u32) -> Result<bool> {
    let (element_type, bytes_read) = read_u32(bytes)?;
    Ok(if element_type == expected {
        *bytes = seek(bytes, bytes_read);
        true
    } else {
        false
    })
}

fn seek(bytes: &[u8], bytes_read: usize) -> &[u8] {
    bytes.get(bytes_read..).unwrap()
}

fn read_u32<'a>(bytes: &[u8]) -> Result<(u32, usize)> {
    if bytes.is_empty() {
        return Err(unsupported_blob());
    }
    let bytes_read;
    let value = if bytes[0] & 0x80 == 0 {
        bytes_read = 1;
        bytes[0] as u32
    } else if bytes[0] & 0xC0 == 0x80 {
        if bytes.len() < 2 {
            return Err(unsupported_blob());
        }
        bytes_read = 2;
        (((bytes[0] & 0x3F) as u32) << 8) | bytes[1] as u32
    } else if bytes[0] & 0xE0 == 0xC0 {
        if bytes.len() < 4 {
            return Err(unsupported_blob());
        }
        bytes_read = 4;
        (((bytes[0] & 0x1F) as u32) << 24) | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
    } else {
        return Err(unsupported_blob());
    };

    Ok((value, bytes_read))
}

pub fn unsupported_blob() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported blob")
}

pub enum ElementType {
    Void = 0x01,
    Boolean = 0x02,
    Char = 0x03,
    I1 = 0x04,
    U1 = 0x05,
    I2 = 0x06,
    U2 = 0x07,
    I4 = 0x08,
    U4 = 0x09,
    I8 = 0x0a,
    U8 = 0x0b,
    R4 = 0x0c,
    R8 = 0x0d,
    String = 0x0e,

    Ptr = 0x0f,       // Followed by TypeSig
    ByRef = 0x10,     // Followed by TypeSig
    ValueType = 0x11, // Followed by TypeDef or TypeRef
    Class = 0x12,     // Followed by TypeDef or TypeRef
    Var = 0x13,       // Generic parameter in a type definition, represented as unsigned integer
    Array = 0x14,
    GenericInst = 0x15,
    TypedByRef = 0x16,

    I = 0x18, // System.IntPtr
    U = 0x19, // System.UIntPtr

    FnPtr = 0x1b,  // Followed by full method signature
    Object = 0x1c, // System.Object
    SZArray = 0x1d,
    MVar = 0x1e,     // Generic parameter in a method definition, represented as unsigned integer
    CModReqd = 0x1f, // Required modifier, followed by a TypeDef or TypeRef
    CModOpt = 0x20,  // Optional modifier, followed by a TypeDef or TypeRef
    Internal = 0x21,

    Modifier = 0x40, // Or'd with folowing element types
    Sentinel = 0x41, // Sentinel for vararg method signature

    Pinned = 0x45,

    Type = 0x50,         // System.Type
    TaggedObject = 0x51, // Boxed object (in custom attributes)
    Field = 0x53,        // Custom attribute field
    Property = 0x54,     // Custom attribute property
    Enum = 0x55,         // Custom attribute enum
}
impl std::fmt::Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementType::Boolean => write!(f, "bool"),
            _ => write!(f, "..ElementType.."),
        }
    }
}