extern crate memmap;

use std::error::Error;
use std::fs::File;
use memmap::Mmap;

mod header;
use header::*;


/* Type for a 16-bit quantity. */
pub type Elf32Half = [u8; 2];  // u16
pub type Elf64Half = [u8; 2];  // u16

/* Types for signed and unsigned 32-bit quantities. */
pub type Elf32Word = [u8; 4];  // u32
pub type Elf32Sword = [u8; 4];  // i32
pub type Elf64Word = [u8; 4];  // u32
pub type Elf64Sword = [u8; 4];  // i32

/* Types for signed and unsigned 64-bit quantities. */
pub type Elf64Xword = [u8; 8];  // u64
pub type Elf64Sxword = [u8; 8];  // i64

/* Type of addresses. */
pub type Elf32Addr = [u8; 4];  // u32
pub type Elf64Addr = [u8; 8];  // u64

/* Type of file offsets. */
pub type Elf32Off = [u8; 4];  // u32
pub type Elf64Off = [u8; 8];  // u64

/* Type for section indices, which are 16-bit quantities. */
pub type Elf64Section = [u8; 2];  // u16

/* Type for version symbol information.  */
pub type Elf64Versym = Elf64Half;  // u16


pub fn run(filename: &str) -> Result<(), Box<dyn Error>>{
    let file = File::open(filename).unwrap();
    let mapped_file = unsafe { Mmap::map(&file).unwrap() };

    let mut e_ident: [u8; 16] = [0; 16];
    for (i, b) in mapped_file[0..16].iter().enumerate() {
        e_ident[i] = *b;
    }
    if is_elf(e_ident) {
        match &*ei_class(e_ident) {
            "?" => println!("I don't know"),
            "ELF32" => show_elf32_hdr(Elf32Hdr::new(&mapped_file)),
            "ELF64" => show_elf64_hdr(Elf64Hdr::new(&mapped_file)),
            _ => println!("Wrong format"),
        }
    }

    Ok(())
}