use std::fs::File;
use std::io::{BufReader, Read};

struct CPInfo {
    tag: u8,
    info: Vec<u8>,
}

#[derive(Debug, PartialEq)]
enum ConstantPool {
    // u1 tag; u2 name_index;
    Class(u16),
}

// WIP
#[derive(Debug)]
struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pools: Vec<ConstantPool>,
}

struct ClassFileReader {
    reader: BufReader<File>,
}

type u1 = u8;
type u2 = u16;
type u4 = u32;

impl ClassFileReader {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let reader = BufReader::new(File::open(filename)?);
        Ok(Self { reader })
    }

    fn read(&mut self) -> Option<ClassFile> {
        let magic = self.read_4byte();
        if magic != 0xCAFEBABE {
            return None;
        }
        let minor_version = self.read_2byte();
        let major_version = self.read_2byte();
        let constant_pool_count = self.read_2byte();

        let constant_pools = {
            let mut v = vec![];
            for _ in 0..constant_pool_count {
                v.push(self.read_constant_pool());
            }
            v
        };
        Some(ClassFile { minor_version, major_version, constant_pool_count, constant_pools})
    }

    fn read_constant_pool(&mut self) -> ConstantPool {
        use ConstantPool::*;
        let tag = self.read_2byte();
        match tag {
            7 => Class(self.read_2byte()),
        }
    }

    fn read_4byte(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        match self.reader.read(&mut buf) {
            Ok(_) => {
                ((buf[0] as u32) << 24)
                    + ((buf[1] as u32) << 16)
                    + ((buf[2] as u32) << 8)
                    + buf[3] as u32
            }
            _ => 0,
        }
    }

    fn read_2byte(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        match self.reader.read(&mut buf) {
            Ok(_) => {
                ((buf[0] as u16) << 8)
                    + buf[1] as u16
            }
            _ => 0,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let filename = std::env::args().nth(1).unwrap();
    let mut class_file_reader = ClassFileReader::new(&filename)?;
    let classfile = class_file_reader.read();
    println!("{} is Java class file? : {:?}", filename, classfile.is_some());
    if classfile.is_some() {
        println!("{:?}", classfile.unwrap());
    }
    Ok(())
}
