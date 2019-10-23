#![allow(dead_code)]

use crate::codes::*;
use crate::database::*;
use crate::flags::*;
use std::io::Result;

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
    };
}

table!(TypeDef);
impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> Result<TypeAttributes> {
        Ok(TypeAttributes(self.row.u32(0)?))
    }
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.row.str(2)
    }
    pub fn extends(&self) -> Result<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.row.table.db, self.row.u32(3)?))
    }
    pub fn methods(&self) -> Result<RowIterator<'a, MethodDef<'a>>> {
        let first = self.row.u32(5)? - 1;
        let last = if self.row.index + 1 < self.row.table.len() { self.row.table.row::<TypeDef>(self.row.index + 1).row.u32(5)? - 1 } else { self.row.table.db.method_def().len() };
        Ok(RowIterator::new(&self.row.table.db.method_def(), first, last))
    }
    pub fn attributes(&self) -> Result<RowIterator<'a, CustomAttribute<'a>>> {
        self.row.table.db.custom_attribute().equal_range(0, HasCustomAttribute::TypeDef(*self).encode())
    }
    pub fn has_attribute(&self, namespace: &str, name: &str) -> Result<bool> {
        for attribute in self.attributes()? {
            if attribute.has_name(namespace, name)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

table!(TypeRef);
impl<'a> TypeRef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.row.str(2)
    }
}

table!(TypeSpec);

table!(CustomAttribute);
impl<'a> CustomAttribute<'a> {
    pub fn parent(&self) -> Result<HasCustomAttribute> {
        Ok(HasCustomAttribute::decode(&self.row.table.db, self.row.u32(0)?))
    }
    pub fn class(&self) -> Result<CustomAttributeType> {
        Ok(CustomAttributeType::decode(&self.row.table.db, self.row.u32(1)?))
    }

    pub fn has_name(&self, namespace: &str, name: &str) -> Result<bool> {
        Ok(match self.class()? {
            CustomAttributeType::MethodDef(value) => {
                let parent = value.parent()?;
                name == parent.name()? && namespace == parent.namespace()?
            }
            CustomAttributeType::MemberRef(value) => match value.class()? {
                MemberRefParent::TypeDef(value) => name == value.name()? && namespace == value.namespace()?,
                MemberRefParent::TypeRef(value) => name == value.name()? && namespace == value.namespace()?,
                _ => false,
            },
        })
    }
}

table!(MethodDef);
impl<'a> MethodDef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(3)
    }
    pub fn parent(&self) -> Result<TypeDef> {
        self.row.table.db.type_def().upper_bound(6, self.row.index)
    }
}

table!(MemberRef);
impl<'a> MemberRef<'a> {
    pub fn class(&self) -> Result<MemberRefParent> {
        Ok(MemberRefParent::decode(&self.row.table.db, self.row.u32(0)?))
    }
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
}

table!(GenericParamConstraint);
table!(Module);
table!(Param);
table!(InterfaceImpl);
table!(Constant);
table!(Field);
table!(FieldMarshal);
table!(DeclSecurity);
table!(ClassLayout);
table!(FieldLayout);
table!(StandaloneSig);
table!(EventMap);
table!(Event);
table!(PropertyMap);
table!(Property);
table!(MethodSemantics);
table!(MethodImpl);
table!(ModuleRef);
table!(ImplMap);
table!(FieldRva);
table!(Assembly);
table!(AssemblyProcessor);
table!(AssemblyOs);
table!(AssemblyRef);
table!(AssemblyRefProcessor);
table!(AssemblyRefOs);
table!(File);
table!(ExportedType);
table!(ManifestResource);
table!(NestedClass);
table!(GenericParam);
table!(MethodSpec);
