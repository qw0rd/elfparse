use std::fs;
use std::io::prelude::*;

fn main() {
    //let file = fs::read("target/debug/libelfparse.d").unwrap();
    let data = fs::read("target/debug/examples/parse").unwrap();
    let file = elfparse::ElfFile::from_bytes(&data).unwrap();

    println!("{:?}", file.header);

    let mut sections = file.sections();
    let text_sec = file.lookup_section(".text");
}
