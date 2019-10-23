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

table!(Assembly);
table!(AssemblyOs);
table!(AssemblyProcessor);
table!(AssemblyRef);
table!(AssemblyRefOs);
table!(AssemblyRefProcessor);
table!(ClassLayout);
table!(Constant);
table!(CustomAttribute);
table!(DeclSecurity);
table!(Event);
table!(EventMap);
table!(ExportedType);
table!(Field);
table!(FieldLayout);
table!(FieldMarshal);
table!(FieldRva);
table!(File);
table!(GenericParam);
table!(GenericParamConstraint);
table!(ImplMap);
table!(InterfaceImpl);
table!(ManifestResource);
table!(MemberRef);
table!(MethodDef);
table!(MethodImpl);
table!(MethodSemantics);
table!(MethodSpec);
table!(Module);
table!(ModuleRef);
table!(NestedClass);
table!(Param);
table!(Property);
table!(PropertyMap);
table!(StandaloneSig);
table!(TypeDef);
table!(TypeRef);
table!(TypeSpec);

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

impl<'a> MemberRef<'a> {
    pub fn class(&self) -> Result<MemberRefParent> {
        Ok(MemberRefParent::decode(&self.row.table.db, self.row.u32(0)?))
    }
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
}

impl<'a> MethodDef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(3)
    }
    pub fn parent(&self) -> Result<TypeDef> {
        self.row.table.db.type_def().upper_bound(6, self.row.index)
    }
}

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
        self.row.list(5, &self.row.table.db.method_def())
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

impl<'a> TypeRef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.row.str(2)
    }
}
