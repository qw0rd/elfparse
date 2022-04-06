
use std::fs;
use libc;

fn main() {
    let data = fs::read("a.out").unwrap();
    let info = elfparse::ElfFile::from_bytes(&data).unwrap();

    let text = info.lookup_section(".text").unwrap();
    println!("{:?}", text);

    let _ptr = unsafe {
        libc::mmap(core::ptr::null_mut(),
        4096,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, 
        libc::MAP_PRIVATE | libc::MAP_ANON,
        -1,0)
    };

    let ptr = unsafe { core::slice::from_raw_parts_mut(_ptr as *mut u8, 4096) };
    let mut i = 0usize;

    while i < (text.sh_size as usize) {
        ptr[i] = data[text.sh_offset as usize + i];
        i += 1;
    }

    let addr = (_ptr as usize) + (info.header.e_entry - (text.sh_offset as usize));
    let func = unsafe { core::mem::transmute::<usize, GG>(addr) } ;
    let res = func();
    println!("{}", res);
}

type GG = extern fn() -> i32;
