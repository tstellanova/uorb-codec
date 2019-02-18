#![recursion_limit="256"]
#[macro_use]
extern crate quote;


use std::env;
use std::fs::{File};
use std::path::{Path};

//import the custom uORB message parser
mod parser;



pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("common.rs");
    let mut fout = File::create(&dest_path).unwrap();

    let src_dir = env::current_dir().unwrap();
    let msg_dr = src_dir.join("msg");

    let mut parso = parser::Parser::new();
    let _res = parso.process_msg_directory(&msg_dr, &mut fout);

    println!("output path: {:?}", dest_path);

    //TODO verify this works if we add new files OR updated files
    println!("cargo:rerun-if-changed=msg/");
}
