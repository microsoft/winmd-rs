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

pub struct TypeAttributes(pub(crate) u32);

impl TypeAttributes {
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

    pub fn windows_runtime(&self) -> bool {
        self.0 & (1 << 14) != 0
    }
}
