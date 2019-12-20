use std::fs::File;
use std::io::{BufReader, Read};

struct CPInfo {
    tag: u8,
    info: Vec<u8>,
}

// WIP
struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    cp_info: CPInfo,
}

struct ClassFileReader {
    reader: BufReader<File>,
}

impl ClassFileReader {
    fn new(filename: &str) -> Result<Self, std::io::Error> {
        let reader = BufReader::new(File::open(filename)?);
        Ok(Self { reader })
    }

    fn has_magic(&mut self) -> bool {
        let magic = self.read_4byte();
        magic == 0xCAFEBABE
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
}

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let filename = std::env::args().nth(1).unwrap();
    let mut class_file_reader = ClassFileReader::new(&filename)?;
    println!("{} is Java class file? : {}", filename, class_file_reader.has_magic());
    Ok(())
}
