// enum TypeVisibility {
//     NotPublic,
//     Public,
//     NestedPublic,
//     NestedPrivate,
//     NestedFamily,
//     NestedAssembly,
//     NestedFamANDAssem,
//     NestedFamORAssem,
// }

pub struct MethodAttributes(pub(crate) u32);
pub struct TypeAttributes(pub(crate) u32);

impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0b100000000000 != 0
    }
}

impl TypeAttributes {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100000000000000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b100000 != 0
    }
    // fn visibility(&self) -> TypeVisibility {
    //     match self.0 & 0b111 {
    //         1 => TypeVisibility::Public,
    //         2 => TypeVisibility::NestedPublic,
    //         3 => TypeVisibility::NestedPrivate,
    //         4 => TypeVisibility::NestedFamily,
    //         5 => TypeVisibility::NestedAssembly,
    //         6 => TypeVisibility::NestedFamANDAssem,
    //         7 => TypeVisibility::NestedFamORAssem,
    //         _ => TypeVisibility::NotPublic,
    //     }
    // }
}
