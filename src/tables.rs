use crate::database::*;
use crate::flags::*;

//
// TypeDef
//

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
    db: &'a Database,
    index: u32,
}

impl<'a> TypeDefRow<'a> {
    pub fn flags(&self) -> std::io::Result<TypeAttributes> {
        Ok(TypeAttributes(self.db.u32(&self.db.type_def, self.index, 0)?))
    }
    pub fn name(&self) -> std::io::Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 1)
    }
    pub fn namespace(&self) -> std::io::Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 2)
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

//
// TypeRef
//

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
    db: &'a Database,
    index: u32,
}

impl<'a> TypeRefRow<'a> {
    pub fn name(&self) -> std::io::Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 1)
    }
    pub fn namespace(&self) -> std::io::Result<&'a str> {
        self.db.string(&self.db.type_def, self.index, 2)
    }
}

impl<'a> Iterator for TypeRefRow<'a> {
    type Item = TypeRefRow<'a>;

    fn next(&mut self) -> Option<TypeRefRow<'a>> {
        if self.index >= self.db.type_def.rows() {
            return None;
        }

        let result = Some(*self);
        self.index += 1;
        result
    }
}
