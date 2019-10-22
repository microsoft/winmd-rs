#![allow(dead_code)]

use crate::database::*;
use crate::flags::*;
use crate::codes::*;
use std::io::Result;

macro_rules! table {
    ($name:ident) => {
        pub struct $name<'a> {
            pub(crate) data: RowData<'a>,
        }
        impl<'a> Row<'a> for $name<'a> {
            fn new(table: &'a Table<'a>, index: u32) -> Self {
                Self { data: RowData { table, index } }
            }
        }
    };
}

table!(TypeDef);
impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> Result<TypeAttributes> {
        Ok(TypeAttributes(self.data.u32(0)?))
    }
    pub fn name(&self) -> Result<&str> {
        self.data.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.data.str(2)
    }
}

table!(TypeRef);
impl<'a> TypeRef<'a> {
    pub fn name(&self) -> Result<&str> {
        self.data.str(1)
    }
    pub fn namespace(&self) -> Result<&str> {
        self.data.str(2)
    }
     pub fn extends(&self) -> Result<TypeDefOrRef> {
         Ok(TypeDefOrRef::decode(&self.data.table.db, self.data.u32(3)?))
     }
//     pub fn methods(&self) -> Result<MethodDef> {
//         self.list::<MethodDef>(5)
//     }
//     pub fn attributes(&self) -> Result<CustomAttribute<'a>> {
//         let parent = HasCustomAttribute::TypeDef2(*self);
//         let (first, last) = self.db.equal_range(&self.db.custom_attribute, 0, self.db.custom_attribute.rows(), 0, parent.encode())?;
//         Ok(CustomAttribute::range(self.db, first, last))
//     }
//     pub fn has_attribute(&self, namespace: &str, name: &str) -> Result<bool> {
//         for attribute in self.attributes()? {
//             if attribute.has_name(namespace, name)? {
//                 return Ok(true);
//             }
//         }
//         Ok(false)
//     }
}

table!(TypeSpec);


// table!(custom_attribute, CustomAttribute);
// impl<'a> CustomAttribute<'a> {
//     pub fn parent(&self) -> Result<HasCustomAttribute> {
//         Ok(HasCustomAttribute::decode(&self.db, self.u32(0)?))
//     }
//     pub fn class(&self) -> Result<CustomAttributeType> {
//         Ok(CustomAttributeType::decode(&self.db, self.u32(1)?))
//     }
//     // value() -> Result<CustomAttributeSig>

//     pub fn has_name(&self, namespace: &str, name: &str) -> Result<bool> {
//         Ok(match self.class()? {
//             CustomAttributeType::MethodDef(row) => {
//                 let parent = row.parent()?;
//                 name == parent.name()? && namespace == parent.namespace()?
//             }
//             CustomAttributeType::MemberRef(row) => match row.class()? {
//                 MemberRefParent::TypeDef2(row) => name == row.name()? && namespace == row.namespace()?,
//                 MemberRefParent::TypeRef(row) => name == row.name()? && namespace == row.namespace()?,
//                 _ => false,
//             },
//         })
//     }
// }

// table!(method_def, MethodDef);
// impl<'a> MethodDef<'a> {
//     pub fn name(&self) -> Result<&'a str> {
//         self.str(3)
//     }
//     pub fn parent(&self) -> Result<TypeDef2> {
//         let last = self.db.type_def.rows();
//         let first = self.db.upper_bound(&self.db.type_def, 0, last, 6, self.first)?;
//         Ok(TypeDef2::range(self.db, first, last))
//     }
// }

// table!(member_ref, MemberRef);
// impl<'a> MemberRef<'a> {
//     pub fn class(&self) -> Result<MemberRefParent> {
//         Ok(MemberRefParent::decode(&self.db, self.u32(0)?))
//     }
//     pub fn name(&self) -> Result<&'a str> {
//         self.str(1)
//     }
//     // pub fun signature(&self) {}
// }
