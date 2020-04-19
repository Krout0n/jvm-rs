use crate::class::{classfile::ClassFile, constant_pool::ConstantPool, method::MethodInfo};
use ConstantPool::*;

#[derive(Debug)]
pub struct VM {
    constant_pools: Vec<ConstantPool>,
}

impl From<ClassFile> for VM {
    fn from(c: ClassFile) -> Self {
        return Self {
            constant_pools: c.constant_pools,
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
    pub fn exec(&mut self, methods: Vec<MethodInfo>) -> Option<&MethodInfo> {
        let mut main = 0;
        for (i, method) in methods.iter().enumerate() {
            if dbg!(self.get_method_name(method)) == "main" {
                main = i;
            }
        }
        self.exec_method(&methods.get(main).unwrap().clone());
        None
    }

    fn exec_method(&mut self, method: &MethodInfo) {
        dbg!(method);
    }
}
