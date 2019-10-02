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

#[derive(Copy, Clone)]
pub struct TypeDef<'a> {
    pub(crate) db: &'a Database,
}
impl<'a> IntoIterator for TypeDef<'a> {
    type Item = TypeDefRow<'a>;
    type IntoIter = TypeDefRow<'a>;
    fn into_iter(self) -> TypeDefRow<'a> {
        TypeDefRow { db: self.db, index: 0 }
    }
}
#[derive(Copy, Clone)]
pub struct TypeDefRow<'a> {
    pub(crate) db: &'a Database,
    pub(crate) index: u32,
}
impl<'a> TypeDefRow<'a> {
    pub fn flags(&self) -> Result<TypeAttributes> {
        Ok(TypeAttributes(self.db.u32(&self.db.type_def, self.index, 0)?))
    }
    pub fn name(&self) -> Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 2)
    }
    pub fn extends(&self) -> Result<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.db, self.db.u32(&self.db.type_def, self.index, 3)?))
    }
    pub fn category(&self) -> Result<Category> {
        if self.flags()?.interface() {
            return Ok(Category::Interface);
        }
        match self.extends()?.name()? {
            "Enum" => Ok(Category::Enum),
            "ValueType" => {
                // TODO: check when it has ApiContractAttribute and then return Category::Contract
                Ok(Category::Struct)},
            "MulticastDelegate" => Ok(Category::Delegate),
            "Attribute" => Ok(Category::Attribute),
            _ => Ok(Category::Class),
        }
    }
}
impl<'a> Iterator for TypeDefRow<'a> {
    type Item = TypeDefRow<'a>;
    fn next(&mut self) -> Option<TypeDefRow<'a>> {
        if self.index >= self.db.type_def.rows() {
            return None;
        }
        let result = Some(*self);
        self.index += 1;
        result
    }
}

#[derive(Copy, Clone)]
pub struct TypeRef<'a> {
    pub(crate) db: &'a Database,
}
impl<'a> IntoIterator for TypeRef<'a> {
    type Item = TypeRefRow<'a>;
    type IntoIter = TypeRefRow<'a>;
    fn into_iter(self) -> TypeRefRow<'a> {
        TypeRefRow { db: self.db, index: 0 }
    }
}
#[derive(Copy, Clone)]
pub struct TypeRefRow<'a> {
    pub(crate) db: &'a Database,
    pub(crate) index: u32,
}
impl<'a> TypeRefRow<'a> {
    pub fn name(&self) -> Result<&'a str> {
        self.db.string(&self.db.type_ref, self.index, 1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.db.string(&self.db.type_ref, self.index, 2)
    }
}
impl<'a> Iterator for TypeRefRow<'a> {
    type Item = TypeRefRow<'a>;
    fn next(&mut self) -> Option<TypeRefRow<'a>> {
        if self.index >= self.db.type_ref.rows() {
            return None;
        }
        let result = Some(*self);
        self.index += 1;
        result
    }
}
