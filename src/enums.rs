use crate::tables::*;

trait CodedIndex
{
    fn bits(&self) -> u32;
}

enum TypeDefOrRef<'a>
{
    TypeDef(TypeDefRow<'a>),
    TypeRef(TypeRefRow<'a>),
    // TypeSpec,
}

impl<'a> CodedIndex for TypeDefOrRef<'a>
{
    fn bits(&self) -> u32 { 2 }
    // TODO: add decode function that takes a u32.
}
