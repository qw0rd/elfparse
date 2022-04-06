use std::fs;
use std::io::prelude::*;

fn main() {
    //let file = fs::read("target/debug/libelfparse.d").unwrap();
    let data = fs::read("target/debug/examples/parse").unwrap();
    let file = elfparse::ElfFile::from_bytes(&data).unwrap();

    println!("{:?}", file.header);

    let mut sections = file.sections();

    println!("{}", sections.count());

/*
    let section_table = file.sections().nth(file.header.e_shstrndx as usize).unwrap();
    println!("{:?}", section_table);

    let mut curr = section_table.sh_offset as usize;
    let mut from = section_table.sh_offset as usize;
    let to = (from + section_table.sh_size as usize) as usize;

/*
    while curr < to {
        if data[curr] == 0 {
            let str_maybe = &data[from..curr];
            let name = core::str::from_utf8(str_maybe).unwrap();
            println!("{}", name);
            from = curr + 1;
        } 
        curr += 1;
    }
    */
    */

    for section in file.sections() {
        println!("{:?}", file.section_name(&section));
        println!("{:x?}", section);
    }
}
