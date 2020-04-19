use super::common_type::*;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
