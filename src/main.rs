extern crate jvm_rs;

use jvm_rs::class::classfile::ClassFile;

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }
    let filename = std::env::args().nth(1).unwrap();
    let classfile = ClassFile::new(&filename);
    println!("{} is Java class file? : {:?}", filename, classfile.is_ok());
    if classfile.is_ok() {
        let classfile = classfile.unwrap();
        dbg!(classfile.methods);
        // let idx = classfile
        //     .methods
        //     .get(0)
        //     .unwrap()
        //     .attributes
        //     .get(0)
        //     .unwrap()
        //     .attribute_name_index as usize;
        // dbg!(&classfile.constant_pools.get(idx));
        // dbg!(&classfile);
        // let idx = classfile.attributes.get(0).unwrap().attribute_name_index as usize;
        // dbg!(&classfile.constant_pools.get(idx));
        // // let idx = classfile.attributes.get(1).unwrap().attribute_name_index as usize;
        // // dbg!(&classfile.constant_pools.get(idx - 1));
        // // // dbg!(classfile);
        // // // dbg!(classfile.methods);
        // // for method in classfile.methods.iter() {
        // //     let idx = method.name_index as usize;
        // //     dbg!(&classfile.constant_pools.get(idx - 1));
        // // }
    }
    Ok(())
}
