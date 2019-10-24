use crate::error::*;
use std::io::Result;
use crate::tables::*;

pub struct MethodSignature
{
    // return_type : return_type
    // params : param
}

struct ReturnTypeSignature
{

}

struct TypeSignature
{

}

impl MethodSignature{
    pub(crate) fn new(method: &MethodDef<'_>) -> Result<MethodSignature>{

        let blob = method.row.blob(4);

        

        Err(invalid_data("TODO"))
    }
}