use crate::tables::*;
use crate::database::*;

trait CodedIndex
{
    // fn bits(&self) -> u32;
    fn decode<'a>(db: &'a Database, index: u32) -> TypeDefOrRef<'a>;
}

enum TypeDefOrRef<'a>
{
    TypeDef(TypeDefRow<'a>),
    //TypeRef(TypeRefRow<'a>),
    // TypeSpec,
}

impl<'a> CodedIndex<'a> for TypeDefOrRef<'a>
{
    // fn bits(&self) -> u32 { 2 }
    fn decode(db: &'a Database, index: u32) -> TypeDefOrRef<'a>
    {
        match index & ((1 << 2) - 1)
        {
            0 => TypeDefOrRef::TypeDef(TypeDefRow { db: db, index: (index >> 2) - 1 }),
        }
    }
}
