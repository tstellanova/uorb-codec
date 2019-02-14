//#![recursion_limit="256"]
//#[macro_use]
//extern crate quote;


use std::env;
use std::fs::File;
use std::path::Path;

//import the custom uORB message parser
mod parser;



pub fn main() {
    let src_dir = env::current_dir().unwrap();
    //TODO iterate over all the .msg files in the msg directory
    let in_path = Path::new(&src_dir).join("msg").join("actuator_controls.msg");
    let mut fin = File::open(&in_path).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    //this dest path must match with that imported in lib.rs
    let dest_path = Path::new(&out_dir).join("common.rs");
    let mut fout = File::create(&dest_path).unwrap();

    parser::generate(&mut fin, &mut fout);


    //TODO rerun if anything in msg directory changes
    println!("cargo:rerun-if-changed=msg/actuator_controls.msg");

}
