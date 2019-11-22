use crate::codes::*;
use crate::helpers::*;
use crate::error::*;
use crate::file::*;
use crate::flags::*;
use crate::signatures::*;

macro_rules! table {
    ($name:ident) => {
        #[derive(Copy, Clone)]
        pub struct $name<'a> {
            pub(crate) row: RowData<'a>,
        }
        impl<'a> Row<'a> for $name<'a> {
            fn new(table: &Table<'a>, index: u32) -> Self {
                Self { row: RowData { table: *table, index } }
            }
        }
        impl<'a> PartialEq for $name<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.row.index == other.row.index && self.row.table.file == other.row.table.file
            }
        }
    };
}

table!(Constant);
table!(CustomAttribute);
table!(Field);
table!(GenericParam);
table!(InterfaceImpl);
table!(MemberRef);
table!(MethodDef);
table!(Param);
table!(TypeDef);
table!(TypeRef);
table!(TypeSpec);

pub enum ConstantValue {
    I32(i32),
    U32(u32),
}

impl std::fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantValue::U32(value) => write!(f, "{}", value),
            ConstantValue::I32(value) => write!(f, "{}", value),
        }
    }
}

impl<'a> Constant<'a> {
    pub fn value(&self) -> ParseResult<ConstantValue> {
        match self.row.u32(0)? {
            0x08 => Ok(ConstantValue::I32(*self.row.blob_as::<i32>(2)?)),
            0x09 => Ok(ConstantValue::U32(*self.row.blob_as::<u32>(2)?)),
            _ => Err(ParseError::InvalidFile),
        }
    }
}

impl<'a> CustomAttribute<'a> {
    pub fn parent(&self) -> ParseResult<HasCustomAttribute> {
        Ok(HasCustomAttribute::decode(&self.row.table.file, self.row.u32(0)?)?)
    }

    pub fn constructor(&self) -> ParseResult<CustomAttributeType> {
        Ok(CustomAttributeType::decode(&self.row.table.file, self.row.u32(1)?)?)
    }

    pub fn arguments(&'a self) -> ParseResult<Vec<(&'a str, ArgumentSig)>> {
        Ok(match self.constructor()? {
            CustomAttributeType::MethodDef(value) => ArgumentSig::new(&self.row.table.file, value.row.blob(4)?, self.row.blob(2)?)?,
            CustomAttributeType::MemberRef(value) => ArgumentSig::new(&self.row.table.file, value.row.blob(2)?, self.row.blob(2)?)?,
        })
    }

    pub fn has_name(&self, name: &str) -> ParseResult<bool> {
        let (namespace, name) = split_type_name(name)?;
        Ok(match self.constructor()? {
            CustomAttributeType::MethodDef(value) => {
                let parent = value.parent()?;
                name == parent.name()? && namespace == parent.namespace()?
            }
            CustomAttributeType::MemberRef(value) => match value.parent()? {
                MemberRefParent::TypeDef(value) => name == value.name()? && namespace == value.namespace()?,
                MemberRefParent::TypeRef(value) => name == value.name()? && namespace == value.namespace()?,
                _ => false,
            },
        })
    }
}

impl<'a> Field<'a> {
    pub fn name(&self) -> ParseResult<&str> {
        self.row.str(1)
    }

    pub fn constants(&self) -> ParseResult<RowIterator<'a, Constant<'a>>> {
        self.row.table.file.constant().equal_range(1, HasConstant::Field(*self).encode())
    }
}

impl<'a> MemberRef<'a> {
    pub fn parent(&self) -> ParseResult<MemberRefParent> {
        Ok(MemberRefParent::decode(&self.row.table.file, self.row.u32(0)?)?)
    }

    pub fn name(&self) -> ParseResult<&str> {
        self.row.str(1)
    }
}

impl<'a> MethodDef<'a> {
    pub fn flags(&self) -> ParseResult<MethodAttributes> {
        Ok(MethodAttributes(self.row.u32(2)?))
    }

    pub fn abi_name(&self) -> ParseResult<&str> {
        self.row.str(3)
    }

    pub(crate) fn params(&self) -> ParseResult<RowIterator<'a, Param<'a>>> {
        self.row.list(5, &self.row.table.file.param())
    }

    pub fn parent(&self) -> ParseResult<TypeDef> {
        self.row.table.file.type_def().upper_bound(6, self.row.index)
    }

    pub fn signature(&self) -> ParseResult<MethodSig> {
        MethodSig::new(self)
    }

    pub fn name(&self) -> ParseResult<String> {
        // TODO: need to account for OverloadAttribute considering that Rust doesn't support overloads.

        let mut source = self.abi_name()?;
        let mut result = String::with_capacity(source.len() + 2);

        if self.flags()?.special() {
            if source.starts_with("get_") || source.starts_with("add_") {
                source = &source[4..];
            } else if source.starts_with("put_") {
                result.push_str("set_");
                source = &source[4..];
            } else if source.starts_with("remove_") {
                result.push_str("revoke_");
                source = &source[7..];
            }
        }

        let mut last = false;
        for c in source.chars() {
            if c.is_uppercase() {
                if last {
                    result.push('_');
                    last = false;
                }

                for lower in c.to_lowercase() {
                    result.push(lower);
                }
            } else {
                result.push(c);
                last = true;
            }
        }

        if result.starts_with("get_") {
            result.replace_range(0..4, "");
        }
        Ok(result)
    }
}

impl<'a> Param<'a> {
    pub fn sequence(&self) -> ParseResult<u32> {
        self.row.u32(1)
    }

    pub fn name(&self) -> ParseResult<&str> {
        self.row.str(2)
    }
}

impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> ParseResult<TypeAttributes> {
        Ok(TypeAttributes(self.row.u32(0)?))
    }

    pub fn name(&self) -> ParseResult<&str> {
        self.row.str(1)
    }

    pub fn namespace(&self) -> ParseResult<&str> {
        self.row.str(2)
    }

    pub fn extends(&self) -> ParseResult<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.row.table.file, self.row.u32(3)?)?)
    }

    pub fn fields(&self) -> ParseResult<RowIterator<'a, Field<'a>>> {
        self.row.list(4, &self.row.table.file.field())
    }

    pub fn methods(&self) -> ParseResult<RowIterator<'a, MethodDef<'a>>> {
        self.row.list(5, &self.row.table.file.method_def())
    }

    pub fn attributes(&self) -> ParseResult<RowIterator<'a, CustomAttribute<'a>>> {
        self.row.table.file.custom_attribute().equal_range(0, HasCustomAttribute::TypeDef(*self).encode())
    }

    pub fn has_attribute(&self, name: &str) -> ParseResult<bool> {
        for attribute in self.attributes()? {
            if attribute.has_name(name)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn find_attribute(&self, name: &str) -> ParseResult<CustomAttribute<'a>> {
        for attribute in self.attributes()? {
            if attribute.has_name(name)? {
                return Ok(attribute);
            }
        }
        Err(ParseError::MissingAttribute)
    }
}

impl<'a> TypeRef<'a> {
    pub fn name(&self) -> ParseResult<&str> {
        self.row.str(1)
    }

    pub fn namespace(&self) -> ParseResult<&str> {
        self.row.str(2)
    }
}
