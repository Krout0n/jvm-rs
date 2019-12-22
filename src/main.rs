extern crate jvm_rs;

use jvm_rs::classfile::ClassFile;

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let filename = std::env::args().nth(1).unwrap();
    let classfile = ClassFile::new(&filename);
    println!(
        "{} is Java class file? : {:?}",
        filename,
        classfile.is_ok()
    );
    if classfile.is_ok() {
        let classfile = classfile.unwrap();
        dbg!(classfile);
    }
    Ok(())
}
