use crate::class::{classfile::ClassFile, constant_pool::ConstantPool, method::MethodInfo};
use ConstantPool::*;

#[derive(Debug)]
pub struct VM {
    constant_pools: Vec<ConstantPool>,
    methods: Vec<MethodInfo>,
}

impl From<ClassFile> for VM {
    fn from(c: ClassFile) -> Self {
        return Self {
            constant_pools: c.constant_pools,
            methods: c.methods,
        };
    }
}

impl VM {
    pub fn get_method_name(&self, method: &MethodInfo) -> &str {
        match self
            .constant_pools
            .get(method.name_index as usize - 1)
            .unwrap()
        {
            Utf8 { bytes, length: _ } => &bytes,
            _ => unreachable!(),
        }
    }
    pub fn exec(&mut self) -> Option<&MethodInfo> {
        for method in self.methods.iter() {
            if dbg!(self.get_method_name(method)) == "main" {
                return Some(method);
            }
        }
        None
    }
}
