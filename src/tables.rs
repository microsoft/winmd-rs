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
    //     pub fn methods(&self) -> Result<MethodDef> {
    //         self.list::<MethodDef>(5)
    //     }
    pub fn attributes(&self) -> Result<RowIterator<'a, CustomAttribute<'a>>> {
        self.row.table.db.custom_attribute().equal_range(0, HasCustomAttribute::TypeDef(*self).encode())
    }
    //     pub fn has_attribute(&self, namespace: &str, name: &str) -> Result<bool> {
    //         for attribute in self.attributes()? {
    //             if attribute.has_name(namespace, name)? {
    //                 return Ok(true);
    //             }
    //         }
    //         Ok(false)
    //     }
}

table!(TypeRef);
impl<'a> TypeRef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.row.str(2)
    }
    pub fn extends(&self) -> Result<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.row.table.db, self.row.u32(3)?))
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
//     // value() -> Result<CustomAttributeSig>

    // pub fn has_name(&self, namespace: &str, name: &str) -> Result<bool> {
    //     Ok(match self.class()? {
    //         CustomAttributeType::MethodDef(row) => {
    //             let parent = row.parent()?;
    //             name == parent.name()? && namespace == parent.namespace()?
    //         }
    //         CustomAttributeType::MemberRef(row) => match row.class()? {
    //             MemberRefParent::TypeDef(row) => name == row.name()? && namespace == row.namespace()?,
    //             MemberRefParent::TypeRef(row) => name == row.name()? && namespace == row.namespace()?,
    //             _ => false,
    //         },
    //     })
    // }
 }

table!(MethodDef);
 impl<'a> MethodDef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.row.str(3)
    }
//     pub fn parent(&self) -> Result<TypeDef2> {
//         let last = self.db.type_def.rows();
//         let first = self.db.upper_bound(&self.db.type_def, 0, last, 6, self.first)?;
//         Ok(TypeDef2::range(self.db, first, last))
//     }
 }

table!(MemberRef);
// impl<'a> MemberRef<'a> {
//     pub fn class(&self) -> Result<MemberRefParent> {
//         Ok(MemberRefParent::decode(&self.db, self.u32(0)?))
//     }
//     pub fn name(&self) -> Result<&'a str> {
//         self.str(1)
//     }
//     // pub fun signature(&self) {}
// }

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
