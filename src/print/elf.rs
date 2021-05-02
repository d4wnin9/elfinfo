use crate::*;
use std::fs::File;
use std::io::Read;
use std::error::Error;


fn read_file_as_byte(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(filename)
        .expect("file not found");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .expect("something went wrong reading the file");

    Ok(buf)
}

pub fn ehdr(filename: &str) -> Result<(), Box<dyn Error>>{
    let buf: Vec<u8> = read_file_as_byte(filename)?;
    let elf_class = header::EiClass::from(buf[4]);
    match elf_class {
        header::EiClass::Elf32 => ehdr32(&buf)?,
        header::EiClass::Elf64 => ehdr64(&buf)?,
        _ => ehdr64(&buf)?,
    };

    Ok(())
}

fn ehdr32(buf: &[u8]) -> Result<(), Box<dyn Error>> {
    let ehdr = parser::ehdr(&buf)?.as_32bit();
    println!("ELF Header:");
    println!("  Magic:   {}", ehdr.e_ident.iter().map(|n| format!("{:02X} ", n)).collect::<String>());
    println!("  Class:                             {:?}", header::EiClass::from(ehdr.e_ident[4]));
    println!("  Data:                              {:?}", header::EiData::from(ehdr.e_ident[5]));
    println!("  Version:                           {:?}", header::EiVersion::from(ehdr.e_ident[6]));
    println!("  OS/ABI:                            {:?}", header::EiOsAbi::from(ehdr.e_ident[7]));
    println!("  Type:                              {:?}", header::EType::from(ehdr.e_type));
    println!("  Machine:                           {:?}", header::EMachine::from(ehdr.e_machine));
    println!("  Entry point address:               0x{:X}", ehdr.e_entry);
    println!("  Start of program headers:          0x{:X} (bytes into file)", ehdr.e_phoff);
    println!("  Start of section headers:          0x{:X} (bytes into file)", ehdr.e_shoff);
    println!("  Flags:                             0x{:X}", ehdr.e_flags);
    println!("  Size of this header:               0x{:X} (bytes)", ehdr.e_hsize);
    println!("  Size of program headers:           0x{:X} (bytes)", ehdr.e_phsize);
    println!("  Number of program headers:         0x{:X}", ehdr.e_phnum);
    println!("  Size of section headers:           0x{:X} (bytes)", ehdr.e_shsize);
    println!("  Number of section headers:         0x{:X}", ehdr.e_shnum);
    println!("  Section header string table index: 0x{:X}", ehdr.e_shstrndx);

    Ok(())
}

fn ehdr64(buf: &[u8]) -> Result<(), Box<dyn Error>> {
    let ehdr = parser::ehdr(&buf)?.as_64bit();
    println!("ELF Header:");
    println!("  Magic:   {}", ehdr.e_ident.iter().map(|n| format!("{:02X} ", n)).collect::<String>());
    println!("  Class:                             {:?}", header::EiClass::from(ehdr.e_ident[4]));
    println!("  Data:                              {:?}", header::EiData::from(ehdr.e_ident[5]));
    println!("  Version:                           {:?}", header::EiVersion::from(ehdr.e_ident[6]));
    println!("  OS/ABI:                            {:?}", header::EiOsAbi::from(ehdr.e_ident[7]));
    println!("  Type:                              {:?}", header::EType::from(ehdr.e_type));
    println!("  Machine:                           {:?}", header::EMachine::from(ehdr.e_machine));
    println!("  Entry point address:               0x{:X}", ehdr.e_entry);
    println!("  Start of program headers:          0x{:X} (bytes into file)", ehdr.e_phoff);
    println!("  Start of section headers:          0x{:X} (bytes into file)", ehdr.e_shoff);
    println!("  Flags:                             0x{:X}", ehdr.e_flags);
    println!("  Size of this header:               0x{:X} (bytes)", ehdr.e_hsize);
    println!("  Size of program headers:           0x{:X} (bytes)", ehdr.e_phsize);
    println!("  Number of program headers:         0x{:X}", ehdr.e_phnum);
    println!("  Size of section headers:           0x{:X} (bytes)", ehdr.e_shsize);
    println!("  Number of section headers:         0x{:X}", ehdr.e_shnum);
    println!("  Section header string table index: 0x{:X}", ehdr.e_shstrndx);

    Ok(())
}