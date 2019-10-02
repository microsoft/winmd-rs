use crate::codes::*;
use crate::database::*;
use crate::flags::*;
use std::io::Result;

pub enum Category {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
    Contract,
}

macro_rules! table {
    ($snake:ident, $camel:ident, $row:ident) => {
        #[derive(Copy, Clone)]
        pub struct $camel<'a> {
            pub(crate) db: &'a Database,
        }
        impl<'a> IntoIterator for $camel<'a> {
            type Item = $row<'a>;
            type IntoIter = $row<'a>;
            fn into_iter(self) -> $row<'a> {
                $row { db: self.db, index: 0 }
            }
        }
        #[derive(Copy, Clone)]
        pub struct $row<'a> {
            pub(crate) db: &'a Database,
            pub(crate) index: u32,
        }
        impl<'a> Iterator for $row<'a> {
            type Item = $row<'a>;
            fn next(&mut self) -> Option<$row<'a>> {
                if self.index >= self.db.$snake.rows() {
                    return None;
                }
                let result = Some(*self);
                self.index += 1;
                result
            }
        }
        impl<'a> $row<'a> {
            fn u32(&self, column: u32) -> Result<u32> {
                self.db.u32(&self.db.$snake, self.index, column)
            }
            fn str(&self, column: u32) -> Result<&'a str> {
                self.db.str(&self.db.$snake, self.index, column)
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
table!(method_def, MethodDef, MethodDefRow);
table!(member_ref, MemberRef, MemberRefRow);
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
