use crate::class::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct VM {
    constant_pools: Vec<ConstantPool>
}

impl VM {
    pub fn new(constant_pools: Vec<ConstantPool>) -> Self {
        Self {
            constant_pools
        }
    }
}