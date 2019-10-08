#![allow(dead_code)]

use crate::codes::*;
use crate::database::*;
use crate::flags::*;
use std::io::Result;

#[derive(PartialEq)]
pub enum Category {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
    Contract,
}

trait TableRange<'a> {
    // TODO: maybe use Rust's range parameter syntax here to combine these into one function
    fn range(db: &'a Database, first: u32, last: u32) -> Self;
    fn rest(db: &'a Database, first: u32) -> Self;
}

macro_rules! table {
    ($snake:ident, $camel:ident, $row:ident) => {
        #[derive(Copy, Clone)]
        pub struct $camel<'a> {
            pub(crate) db: &'a Database,
        }
        impl<'a> $camel<'a> {
            pub(crate) fn new(db: &'a Database, index: u32) -> $row<'a> {
                $row { db, first: index, last: index + 1 }
            }
            //fn upper_bound
        }
        impl<'a> IntoIterator for $camel<'a> {
            type Item = $row<'a>;
            type IntoIter = $row<'a>;
            fn into_iter(self) -> $row<'a> {
                $row::rest(self.db, 0)
            }
        }
        #[derive(Copy, Clone)]
        pub struct $row<'a> {
            pub(crate) db: &'a Database,
            pub(crate) first: u32,
            pub(crate) last: u32,
        }
        impl<'a> Iterator for $row<'a> {
            type Item = $row<'a>;
            fn next(&mut self) -> Option<$row<'a>> {
                if self.first >= self.last {
                    return None;
                }
                let result = Some(*self);
                self.first += 1;
                result
            }
        }
        impl<'a> TableRange<'a> for $row<'a> {
            fn range(db: &'a Database, first: u32, last: u32) -> $row<'a> {
                $row { db, first, last }
            }
            fn rest(db: &'a Database, first: u32) -> $row<'a> {
                $row { db, first, last: db.$snake.rows() }
            }
        }
        impl<'a> $row<'a> {
            fn len(&self) -> u32 {
                self.last - self.first
            }
            fn u32(&self, column: u32) -> Result<u32> {
                self.db.u32(&self.db.$snake, self.first, column)
            }
            fn str(&self, column: u32) -> Result<&'a str> {
                self.db.str(&self.db.$snake, self.first, column)
            }
            fn list<T: TableRange<'a>>(&self, column: u32) -> Result<T> {
                let first = self.u32(column)? - 1;

                if self.first + 1 < self.db.$snake.rows() {
                    Ok(T::range(self.db, first, self.db.u32(&self.db.$snake, self.first + 1, column)? - 1))
                } else {
                    Ok(T::rest(self.db, first))
                }
            }
        }
    };
}

table!(type_ref, TypeRef, TypeRefRow);
impl<'a> TypeRefRow<'a> {
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.str(2)
    }
}

table!(generic_param_constraint, GenericParamConstraint, GenericParamConstraintRow);
table!(type_spec, TypeSpec, TypeSpecRow);

table!(type_def, TypeDef, TypeDefRow);
impl<'a> TypeDefRow<'a> {
    pub fn flags(&self) -> Result<TypeAttributes> {
        Ok(TypeAttributes(self.u32(0)?))
    }
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.str(2)
    }
    pub fn extends(&self) -> Result<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.db, self.u32(3)?))
    }
    pub fn methods(&self) -> Result<MethodDefRow> {
        self.list::<MethodDefRow>(5)
    }

    pub fn attributes(&self) -> Result<CustomAttributeRow<'a>>
    {
        let parent = HasCustomAttribute::TypeDef(*self);
        let (first, last) = self.db.equal_range(&self.db.custom_attribute, 0, self.db.custom_attribute.rows(), 0, parent.encode())?;
        Ok(CustomAttributeRow::range(self.db, first, last))
    }

    pub fn category(&self) -> Result<Category> {
        if self.flags()?.interface() {
            return Ok(Category::Interface);
        }
        match self.extends()?.name()? {
            "Enum" => Ok(Category::Enum),
            "ValueType" => {
                // TODO: check when it has ApiContractAttribute and then return Category::Contract
                Ok(Category::Struct)
            }
            "MulticastDelegate" => Ok(Category::Delegate),
            "Attribute" => Ok(Category::Attribute),
            _ => Ok(Category::Class),
        }
    }
}

table!(custom_attribute, CustomAttribute, CustomAttributeRow);
impl<'a> CustomAttributeRow<'a> {
    pub fn parent(&self) -> Result<HasCustomAttribute> {
        Ok(HasCustomAttribute::decode(&self.db, self.u32(0)?))
    }
    pub fn class(&self) -> Result<CustomAttributeType> {
        Ok(CustomAttributeType::decode(&self.db, self.u32(1)?))
    }
    // value() -> Result<CustomAttributeSig>

    // pub fn with_parent(parent: &'a HasCustomAttribute) -> Result<CustomAttributeRow<'a>>
    // {
    //     // TODO: generalize this to reuse this code

    //     let db = parent.database();
    //     let expected = parent.encode();
    //     let mut count = db.custom_attribute.rows();
    //     let mut result = CustomAttributeRow::rest(db, 0);

    //     loop
    //     {
    //         if count <= 0
    //         {
    //             break;
    //         }

    //         let count2 = count / 2;
    //         let middle = result.first + count2;
    //         let actual = db.u32(db.type_def, middle, 0);

    //         if actual < expected // 0 is parent column
    //         {
    //             result.first = middle + 1;
    //             count -= count2 + 1;
    //         }
    //         else if expected < actual{
    //             count = coun2;
    //         }
    //         else{
    //             let first2 = lower_bound
    //         }
    //     }

    //     Ok(result)
    // }

    // pub fn full_name(&self) -> Result<(&str, &str)>
    // {
    //     Ok(match self.class()?
    //     {
    //         CustomAttributeType::MethodDef(row) => {
    //             let parent = row.parent()?;
    //             (parent.namespace()?, parent.name()?)
    //         },
    //         CustomAttributeType::MemberRef(row) =>
    //             match row.class()?
    //             {
    //                 MemberRefParent::TypeDef(row) => (row.namespace()?, row.name()?)
    //             },

    //     })
    // }
}

table!(method_def, MethodDef, MethodDefRow);
impl<'a> MethodDefRow<'a> {
    pub fn name(&self) -> Result<&'a str> {
        self.str(3)
    }
    // pub fn parent(&self) -> Result<TypeDefRow>
    // {
    //     5
    // }
}

table!(member_ref, MemberRef, MemberRefRow);
impl<'a> MemberRefRow<'a> {
    pub fn class(&self) -> Result<MemberRefParent> {
        Ok(MemberRefParent::decode(&self.db, self.u32(0)?))
    }
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    // pub fun signature(&self) {}
}

table!(module, Module, ModuleRow);
table!(param, Param, ParamRow);
table!(interface_impl, InterfaceImpl, InterfaceImplRow);
table!(constant, Constant, ConstantRow);
table!(field, Field, FieldRow);
table!(field_marshal, FieldMarshal, FieldMarshalRow);
table!(decl_security, DeclSecurity, DeclSecurityRow);
table!(class_layout, ClassLayout, ClassLayoutRow);
table!(field_layout, FieldLayout, FieldLayoutRow);
table!(standalone_sig, StandaloneSig, StandaloneSigRow);
table!(event_map, EventMap, EventMapRow);
table!(event, Event, EventRow);
table!(property_map, PropertyMap, PropertyMapRow);
table!(property, Property, PropertyRow);
table!(method_semantics, MethodSemantics, MethodSemanticsRow);
table!(method_impl, MethodImpl, MethodImplRow);
table!(module_ref, ModuleRef, ModuleRefRow);
table!(impl_map, ImplMap, ImplMapRow);
table!(field_rva, FieldRva, FieldRvaRow);
table!(assembly, Assembly, AssemblyRow);
table!(assembly_processor, AssemblyProcessor, AssemblyProcessorRow);
table!(assembly_os, AssemblyOs, AssemblyOsRow);
table!(assembly_ref, AssemblyRef, AssemblyRefRow);
table!(assembly_ref_processor, AssemblyRefProcessor, AssemblyRefProcessorRow);
table!(assembly_ref_os, AssemblyRefOs, AssemblyRefOsRow);
table!(file, File, FileRow);
table!(exported_type, ExportedType, ExportedTypeRow);
table!(manifest_resource, ManifestResource, ManifestResourceRow);
table!(nested_class, NestedClass, NestedClassRow);
table!(generic_param, GenericParam, GenericParamRow);
table!(method_spec, MethodSpec, MethodSpecRow);
