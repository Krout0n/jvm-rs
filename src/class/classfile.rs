#![allow(non_camel_case_types)]

use super::access_flags::*;
use super::attribute::*;
use super::constantpool::*;
use super::field::FieldInfo;
use super::instruction::{Instruction, InstructionKind};
use super::method::MethodInfo;
use std::fs::File;
use std::io::{BufReader, Read};

type u1 = u8;
type u2 = u16;
type u4 = u32;

use RefKind::*;

#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u2,
    pub major_version: u2,
    pub constant_pool_count: u2,
    pub constant_pools: Vec<ConstantPool>,
    pub access_flags: u2,
    pub this_class: u2,
    pub super_class: u2,
    pub interfaces_count: u2,
    pub interfaces: Vec<u2>,
    pub fields_count: u2,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u2,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u2,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct ClassFileReader {
    reader: BufReader<File>,
}

impl ClassFile {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let mut reader = ClassFileReader::new(filename)?;
        if let Some(classfile) = reader.read() {
            /* TODO-Low:
                - Format checking.
                - Check the file satisfies constraints or not.
                - Verification.
            */
            Ok(classfile)
        } else {
            unimplemented!()
        }
    }
}

impl ClassFileReader {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let reader = BufReader::new(File::open(filename)?);
        Ok(Self { reader })
    }

    fn read(&mut self) -> Option<ClassFile> {
        macro_rules! read_n_times {
            ($n: expr, $read_function: ident) => {{
                let mut v = vec![];
                for _ in 0..$n {
                    v.push(self.$read_function());
                }
                v
            }};
        }
        let magic = self.read_u4();
        if magic != 0xCAFEBABE {
            return None;
        }
        let minor_version = self.read_u2();
        let major_version = self.read_u2();
        let constant_pool_count = self.read_u2();
        let constant_pools = read_n_times!(constant_pool_count - 1, read_constant_pool);
        let access_flags = self.read_u2();
        let this_class = self.read_u2();
        let super_class = self.read_u2();
        let interfaces_count = self.read_u2();
        let interfaces = read_n_times!(interfaces_count, read_u2);
        let fields_count = self.read_u2();
        let fields = {
            let mut v = vec![];
            for _ in 0..fields_count {
                let access_flags = self.read_u2();
                let name_index = self.read_u2();
                let descriptor_index = self.read_u2();
                let attributes_count = self.read_u2();
                let attributes = {
                    let mut v = vec![];
                    for _ in 0..attributes_count {
                        v.push(self.read_attribute(&constant_pools));
                    }
                    v
                };
                v.push(FieldInfo {
                    access_flags,
                    name_index,
                    descriptor_index,
                    attributes_count,
                    attributes,
                });
            }
            v
        };
        let methods_count = self.read_u2();
        let methods = {
            let mut v = vec![];
            for _ in 0..methods_count {
                let access_flags = AccessFlags::from(self.read_u2());
                let name_index = self.read_u2();
                let descriptor_index = self.read_u2();
                let attributes_count = self.read_u2();
                let attributes = {
                    let mut v = vec![];
                    println!("methods: attributes");
                    for _ in 0..attributes_count {
                        v.push(self.read_attribute(&constant_pools));
                    }
                    v
                };
                v.push(MethodInfo {
                    access_flags,
                    name_index,
                    descriptor_index,
                    attributes_count,
                    attributes,
                });
            }
            v
        };
        println!("attributes_count");
        let attributes_count = self.read_u2();
        let attributes = {
            let mut v = vec![];
            for _ in 0..attributes_count {
                v.push(self.read_attribute(&constant_pools));
            }
            v
        };
        Some(ClassFile {
            minor_version,
            major_version,
            constant_pool_count,
            constant_pools,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
    }

    fn read_attribute(&mut self, constant_pools: &[ConstantPool]) -> AttributeInfo {
        use ConstantPool::*;
        let attribute_name_index = self.read_u2();
        let attribute_length = self.read_u4();
        let name = match constant_pools
            .get(attribute_name_index as usize - 1)
            .unwrap()
        {
            Utf8 { bytes, .. } => bytes,
            _ => unreachable!(),
        };
        let info = match name.as_str() {
            "Code" => {
                let max_stack = self.read_u2();
                let max_locals = self.read_u2();
                let code_length = self.read_u4();
                let code = {
                    let mut v = vec![];
                    let mut eaten = 0;
                    while eaten < code_length {
                        let kind = InstructionKind::from(self.read_u1());
                        eaten += 1;
                        let argc = kind.argc();
                        let mut args = vec![];
                        if argc == -1 {
                            unimplemented!()
                        } else {
                            for _ in 0..argc {
                                args.push(self.read_u1());
                                eaten += 1;
                            }
                        }
                        v.push(dbg!(Instruction { kind, args }));
                    }
                    // for _ in 0..code_length {
                    //     let code = self.read_u1();
                    // }
                    v
                };
                let exception_table_length = self.read_u2();
                let exception_table = {
                    let mut v = vec![];
                    for _ in 0..exception_table_length {
                        let start_pc = self.read_u2();
                        let end_pc = self.read_u2();
                        let handler_pc = self.read_u2();
                        let catch_type = self.read_u2();
                        v.push(ExceptionTable {
                            start_pc,
                            end_pc,
                            handler_pc,
                            catch_type,
                        });
                    }
                    v
                };
                let attributes_count = self.read_u2();
                let attributes = {
                    let mut v = vec![];
                    for _ in 0..attributes_count {
                        let attr = dbg!(self.read_attribute(constant_pools));
                        v.push(attr);
                    }
                    v
                };
                AttributeInfoKind::Code {
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    attributes_count,
                    attributes,
                }
            }
            "LineNumberTable" => {
                let line_number_table_length = self.read_u2();
                let line_number_table = {
                    let mut v = vec![];
                    for _ in 0..line_number_table_length {
                        let start_pc = self.read_u2();
                        let line_number = self.read_u2();
                        v.push(LineNumberTable {
                            start_pc,
                            line_number,
                        });
                    }
                    v
                };
                AttributeInfoKind::LineNumberTable {
                    line_number_table_length,
                    line_number_table,
                }
            }
            "SourceFile" => AttributeInfoKind::SourceFile(self.read_u2()),
            _ => {
                dbg!(name);
                unimplemented!()
            }
        };
        AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        }
    }

    fn read_constant_pool(&mut self) -> ConstantPool {
        use ConstantPool::*;
        let tag = self.read_u1();
        match tag {
            7 => Class(self.read_u2()),
            9 | 10 | 11 => FieldRef {
                tag: RefKind::from(tag as u8),
                class_index: self.read_u2(),
                name_and_type_index: self.read_u2(),
            },
            8 => String(self.read_u2()),
            3 => Integer(self.read_u4()),
            4 => Float(self.read_u4()),
            5 => Long {
                high: self.read_u4(),
                low: self.read_u4(),
            },
            6 => Double {
                high: self.read_u4(),
                low: self.read_u4(),
            },
            12 => NameAndType {
                descriptor_index: self.read_u2(),
                name_index: self.read_u2(),
            },
            1 => {
                let length = self.read_u2();
                let bytes = std::string::String::from_utf8({
                    let mut v = vec![];
                    for _ in 0..length {
                        v.push(self.read_u1());
                    }
                    v
                })
                .unwrap();
                Utf8 { length, bytes }
            }
            n => {
                eprintln!("header {:?} is unimplemented.", n);
                unimplemented!()
            }
        }
    }

    fn read_u1(&mut self) -> u1 {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => buf[0],
            _ => unimplemented!(),
        }
    }

    fn read_u2(&mut self) -> u2 {
        let mut buf = [0u8; 2];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => ((buf[0] as u2) << 8) + buf[1] as u2,
            err => {
                dbg!(err);
                unimplemented!()
            }
        }
    }

    fn read_u4(&mut self) -> u4 {
        let mut buf = [0u8; 4];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => {
                ((buf[0] as u4) << 24)
                    + ((buf[1] as u4) << 16)
                    + ((buf[2] as u4) << 8)
                    + buf[3] as u4
            }
            _ => unimplemented!(),
        }
    }
}
