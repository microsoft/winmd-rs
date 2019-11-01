pub struct MethodAttributes(pub(crate) u32);
impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0b100000000000 != 0
    }
}

pub struct TypeAttributes(pub(crate) u32);
impl TypeAttributes {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100000000000000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b100000 != 0
    }
}
