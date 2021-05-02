use std::error::Error;


pub mod parser;
pub mod print;
pub mod header;


/* 32-bit ELF base types. */
pub type Elf32Addr = u32;
pub type Elf32Half = u16;
pub type Elf32Off = u32;
pub type Elf32Sword = i32;
pub type Elf32Word = u32;


/* 64-bit ELF base types. */
pub type Elf64Addr = u64;
pub type Elf64Half = u16;
pub type Elf64SHalf = i16;
pub type Elf64Off = u64;
pub type Elf64Sword = i32;
pub type Elf64Word = u32;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;


pub fn run(filename: &str, flag: &str) -> Result<(), Box<dyn Error>> {
    match flag {
        "hdr" => print::ehdr(filename)?,
        _ => println!("WA"),
    }
    Ok(())
}