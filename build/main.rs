#![recursion_limit="256"]
#[macro_use]
extern crate quote;


use std::env;
use std::fs::{self, File};
use std::path::{Path};
//import the custom uORB message parser
mod parser;





fn process_msg_directory(msg_dir: &Path, fout: &mut File )  -> std::io::Result<()> {

    // iterate over all the .msg files in the msg directory

    let msg_file_extension =   ".msg";

    for entry in fs::read_dir(msg_dir)? {
        let entry = entry?;
        println!("processing: {:?}", entry);
        let path = entry.path();

        if !path.is_dir() {
            let fname = path.file_name().unwrap().to_str().unwrap();
            if fname.ends_with(msg_file_extension) {
                println!("hit: {:?}", path);
                let range: usize = fname.len()- msg_file_extension.len();

                let name = fname[..range].to_string();
                let mut fin: File = File::open(path).unwrap();
                parser::generate(name, &mut fin, fout);
            }
            else {
                println!("skip: {:?}", path);
            }
        }
    }

    panic!("eee");


    Ok(())
}

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("common.rs");
    let mut fout = File::create(&dest_path).unwrap();

    let src_dir = env::current_dir().unwrap();
    let msg_dr = src_dir.join("msg");

    let _res = process_msg_directory(&msg_dr, &mut fout);


    //TODO rerun if anything in msg directory changes
//    println!("cargo:rerun-if-changed=msg/actuator_controls.msg");


//    //TODO iterate over all the .msg files in the msg directory
//    let in_path = Path::new(&src_dir).join("msg").join("actuator_controls.msg");
//    let mut fin = File::open(&in_path).unwrap();
//
//    let out_dir = env::var("OUT_DIR").unwrap();
//    //this dest path must match with that imported in lib.rs
//    let dest_path = Path::new(&out_dir).join("common.rs");
//    let mut fout = File::create(&dest_path).unwrap();
//
//    parser::generate(&mut fin, &mut fout);

//
//    //TODO rerun if anything in msg directory changes
//    println!("cargo:rerun-if-changed=msg/actuator_controls.msg");

}
