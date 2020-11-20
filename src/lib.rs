extern crate memmap;


use std::io::{self, Write};
use std::error::Error;
use std::fs::File;
use memmap::Mmap;

mod header;
use header::*;


pub fn run(file: File) -> Result<(), Box<dyn Error>>{
    let mapped_file = unsafe { Mmap::map(&file).unwrap() };
    let hdr = Elf64Hdr::new(&mapped_file);
    show_hdr(hdr);

    Ok(())
}


// elf64 header preview
pub fn show_hdr(hdr: Elf64Hdr) {
    print!("hdr.e_ident: ");
    for x in hdr.e_ident.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_type: ");
    for x in hdr.e_type.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_machine: ");
    for x in hdr.e_machine.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_version: ");
    for x in hdr.e_version.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_entry: ");
    for x in hdr.e_entry.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_phoff: ");
    for x in hdr.e_phoff.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_shoff: ");
    for x in hdr.e_shoff.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_flags: ");
    for x in hdr.e_flags.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_hsize: ");
    for x in hdr.e_hsize.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_phsize: ");
    for x in hdr.e_phsize.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_phnum: ");
    for x in hdr.e_phnum.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_shsize: ");
    for x in hdr.e_shsize.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_shnum: ");
    for x in hdr.e_shnum.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();

    print!("hdr.e_shstrndx: ");
    for x in hdr.e_shstrndx.iter() {
        print!("{:<02x} ", x);
    }
    print!("\n");
    io::stdout().flush().unwrap();
}