use crate::*;
use std::error::Error;


pub fn is_elf(buf: &[u8]) -> bool {
    buf == header::EI_MAG
}

pub fn ehdr(buf: &[u8]) -> Result<header::Ehdr, Box<dyn Error>> {
    let elf_class = header::EiClass::from(buf[4]);

    match elf_class {
        header::EiClass::Elf32 => Ok(header::Ehdr::Ehdr32(header::Ehdr32::new(&buf))),
        header::EiClass::Elf64 => Ok(header::Ehdr::Ehdr64(header::Ehdr64::new(&buf))),
        _ => unimplemented!(),
    }
}