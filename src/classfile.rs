#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::{BufReader, Read};

type u1 = u8;
type u2 = u16;
type u4 = u32;

#[derive(Debug, PartialEq)]
pub enum RefKind {
    Field = 0x9,
    Method,
    Interface,
}

use RefKind::*;

impl From<u8> for RefKind {
    fn from(a: u8) -> Self {
        match a {
            0x9 => Field,
            0xA => Method,
            0xB => Interface,
            _ => {
                eprintln!("{}", a);
                unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConstantPool {
    // u1 tag; u2 name_index;
    // The value of the name_index item must be a valid index into the constant_pool table.
    /*
        #2 = Class              #4             // java/lang/Object
        ...
        #4 = Utf8               java/lang/Object
    */
    Class(u2),
    FieldRef {
        tag: RefKind,
        class_index: u2,
        name_and_type_index: u2,
    },
    String(u2),
    Integer(u4), // bytes
    Float(u4),   // bytes
    Long {
        high: u4,
        low: u4,
    },
    Double {
        high: u4,
        low: u4,
    },
    NameAndType {
        name_index: u2,
        descriptor_index: u2,
    },
    Utf8 {
        length: u2,
        bytes: String,
    },
}

#[derive(Debug)]
pub struct ClassFile {
    minor_version: u2,
    major_version: u2,
    constant_pool_count: u2,
    constant_pools: Vec<ConstantPool>,
    access_flags: u2,
    this_class: u2,
    super_class: u2,
    interfaces_count: u2,
    interfaces: Vec<u2>,
    fields_count: u2,
    fields: Vec<FieldInfo>,
    methods_count: u2,
    methods: Vec<MethodInfo>,
    attributes_count: u2,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
struct FieldInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attributes_count: u2,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
struct AttributeInfo {
    attribute_name_index: u2,
    attribute_length: u4,
    // u1 info[attribute_length];
    info: Vec<u1>,
}

#[derive(Debug)]
struct MethodInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attributes_count: u2,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
struct ClassFileReader {
    reader: BufReader<File>,
}


impl ClassFile {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let mut reader = ClassFileReader::new(filename)?;
        if let Some(classfile) = reader.read() {
            Ok(classfile)
        } else {
            unimplemented!()
        }
    }
}

impl ClassFileReader {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let reader =  BufReader::new(File::open(filename)?);
        Ok(Self { reader })
    }

    fn read(&mut self) -> Option<ClassFile> {
        let magic = self.read_u4();
        if magic != 0xCAFEBABE {
            return None;
        }
        let minor_version = self.read_u2();
        let major_version = self.read_u2();
        let constant_pool_count = self.read_u2();
        let constant_pools = {
            let mut v = vec![];
            for _ in 0..(constant_pool_count - 1) {
                let pool = self.read_constant_pool();
                v.push(pool);
            }
            v
        };
        let access_flags = self.read_u2();
        let this_class = self.read_u2();
        let super_class = self.read_u2();
        let interfaces_count = self.read_u2();
        let interfaces = {
            let mut v = vec![];
            for _ in 0..interfaces_count {
                v.push(self.read_u2());
            }
            v
        };
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
                        v.push(self.read_attribute());
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
                let access_flags = self.read_u2();
                let name_index = self.read_u2();
                let descriptor_index = self.read_u2();
                let attributes_count = self.read_u2();
                let attributes = {
                    let mut v = vec![];
                    for _ in 0..attributes_count {
                        v.push(self.read_attribute());
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
        let attributes_count = self.read_u2();
        let attributes = {
            let mut v = vec![];
            for _ in 0..attributes_count {
                v.push(self.read_attribute());
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

    fn read_attribute(&mut self) -> AttributeInfo {
        let attribute_name_index = self.read_u2();
        let attribute_length = self.read_u4();
        let info = {
            let mut v = vec![];
            for _ in 0..attribute_length {
                v.push(self.read_u1());
            }
            v
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
        match self.reader.read(&mut buf) {
            Ok(_) => buf[0],
            _ => unimplemented!(),
        }
    }

    fn read_u2(&mut self) -> u2 {
        let mut buf = [0u8; 2];
        match self.reader.read(&mut buf) {
            Ok(_) => ((buf[0] as u2) << 8) + buf[1] as u2,
            _ => unimplemented!(),
        }
    }

    fn read_u4(&mut self) -> u4 {
        let mut buf = [0u8; 4];
        match self.reader.read(&mut buf) {
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

