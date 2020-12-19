use crate::*;
use std::usize;
use std::vec::Vec;
use prettytable::Table;


/* elf32 header */
pub struct Elf32Hdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf32Half,
    pub e_machine: Elf32Half,
    pub e_version: Elf32Word,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
    pub e_flags: Elf32Word,
    pub e_hsize: Elf32Half,
    pub e_phsize: Elf32Half,
    pub e_phnum: Elf32Half,
    pub e_shsize: Elf32Half,
    pub e_shnum: Elf32Half,
    pub e_shstrndx: Elf32Half,
}

impl Elf32Hdr {
    pub fn new(bin: &[u8]) -> Elf32Hdr {
        let mut e_ident: [u8; 16] = [0; 16];
        for (i, b) in bin[0..16].iter().enumerate() {
            e_ident[i] = *b;
        }
        let mut e_type: Elf32Half = [0; 2];
        for (i, b) in bin[16..18].iter().enumerate() {
            e_type[i] = *b;
        }
        let mut e_machine: Elf32Half = [0; 2];
        for (i, b) in bin[18..20].iter().enumerate() {
            e_machine[i] = *b;
        }
        let mut e_version: Elf32Word = [0; 4];
        for (i, b) in bin[20..24].iter().enumerate() {
            e_version[i] = *b;
        }
        let mut e_entry: Elf32Addr = [0; 4];
        for (i, b) in bin[24..28].iter().enumerate() {
            e_entry[i] = *b;
        }
        let mut e_phoff: Elf32Off = [0; 4];
        for (i, b) in bin[28..32].iter().enumerate() {
            e_phoff[i] = *b;
        }
        let mut e_shoff: Elf32Off = [0; 4];
        for (i, b) in bin[32..36].iter().enumerate() {
            e_shoff[i] = *b;
        }
        let mut e_flags: Elf32Word = [0; 4];
        for (i, b) in bin[36..40].iter().enumerate() {
            e_flags[i] = *b;
        }
        let mut e_hsize: Elf32Half = [0; 2];
        for (i, b) in bin[40..42].iter().enumerate() {
            e_hsize[i] = *b;
        }
        let mut e_phsize: Elf32Half = [0; 2];
        for (i, b) in bin[42..44].iter().enumerate() {
            e_phsize[i] = *b;
        }
        let mut e_phnum: Elf32Half = [0; 2];
        for (i, b) in bin[44..46].iter().enumerate() {
            e_phnum[i] = *b;
        }
        let mut e_shsize: Elf32Half = [0; 2];
        for (i, b) in bin[46..48].iter().enumerate() {
            e_shsize[i] = *b;
        }
        let mut e_shnum: Elf32Half = [0; 2];
        for (i, b) in bin[48..50].iter().enumerate() {
            e_shnum[i] = *b;
        }
        let mut e_shstrndx: Elf32Half = [0; 2];
        for (i, b) in bin[50..52].iter().enumerate() {
            e_shstrndx[i] = *b;
        }
        Elf32Hdr {
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_hsize,
            e_phsize,
            e_phnum,
            e_shsize,
            e_shnum,
            e_shstrndx,
        }
    }
}

/* elf32 program header */
pub struct Elf32Phdr {
    pub p_type: Elf32Word,
    pub p_offset: Elf32Off,
    pub p_vaddr: Elf32Addr,
    pub p_paddr: Elf32Addr,
    pub p_filesz: Elf32Word,
    pub p_memsz: Elf32Word,
    pub p_flags: Elf32Word,
    pub p_align: Elf32Word,
}

impl Elf32Phdr {
    pub fn new(bin: &[u8], e_phoff: usize) -> Elf32Phdr {
        let mut p_type: Elf32Word = [0; 4];
        for (i, b) in bin[e_phoff..e_phoff+4].iter().enumerate() {
            p_type[i] = *b;
        }
        let mut p_offset: Elf32Off = [0; 4];
        for (i, b) in bin[e_phoff+4..e_phoff+8].iter().enumerate() {
            p_offset[i] = *b;
        }
        let mut p_vaddr: Elf32Addr= [0; 4];
        for (i, b) in bin[e_phoff+8..e_phoff+12].iter().enumerate() {
            p_vaddr[i] = *b;
        }
        let mut p_paddr: Elf32Addr = [0; 4];
        for (i, b) in bin[e_phoff+12..e_phoff+16].iter().enumerate() {
            p_paddr[i] = *b;
        }
        let mut p_filesz: Elf32Word = [0; 4];
        for (i, b) in bin[e_phoff+16..e_phoff+20].iter().enumerate() {
            p_filesz[i] = *b;
        }
        let mut p_memsz: Elf32Word = [0; 4];
        for (i, b) in bin[e_phoff+20..e_phoff+24].iter().enumerate() {
            p_memsz[i] = *b;
        }
        let mut p_flags: Elf32Word = [0; 4];
        for (i, b) in bin[e_phoff+24..e_phoff+28].iter().enumerate() {
            p_flags[i] = *b;
        }
        let mut p_align: Elf32Word = [0; 4];
        for (i, b) in bin[e_phoff+28..e_phoff+32].iter().enumerate() {
            p_align[i] = *b;
        }
        Elf32Phdr {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        }
    }
}


/* elf32 header preview */
pub fn print_elf32_hdr(bin: &[u8]) {
    let hdr = Elf32Hdr::new(&bin);
    println!("ELF Header:");
    println!("  Magic:   {}", ei_magic(hdr.e_ident));
    println!("  Class:                             {}", ei_class(hdr.e_ident));
    println!("  Data:                              {}", ei_data(hdr.e_ident));
    println!("  Version:                           {}", ei_version(hdr.e_ident));
    println!("  OS/ABI:                            {}", ei_osabi(hdr.e_ident));
    println!("  Type:                              {}", elf_type(hdr.e_type));
    println!("  Machine:                           {}", elf_machine(hdr.e_machine));
    println!("  Entry point address:               0x{}", elf32_entry(hdr.e_entry));
    println!("  Start of program headers:          0x{} (bytes into file)", elf32_phoff(hdr.e_phoff));
    println!("  Start of section headers:          0x{} (bytes into file)", elf32_shoff(hdr.e_shoff));
    println!("  Flags:                             0x{}", elf_flags(hdr.e_flags));
    println!("  Size of this header:               0x{} (bytes)", elf_hsize(hdr.e_hsize));
    println!("  Size of program headers:           0x{} (bytes)", elf_phsize(hdr.e_phsize));
    println!("  Number of program headers:         0x{}", elf_phnum(hdr.e_phnum));
    println!("  Size of section headers:           0x{} (bytes)", elf_shsize(hdr.e_shsize));
    println!("  Number of section headers:         0x{}", elf_shnum(hdr.e_shnum));
    println!("  Section header string table index: 0x{}", elf_shstrndx(hdr.e_shstrndx));
}

pub fn print_elf32_phdr(bin: &[u8]) {
    let hdr = Elf32Hdr::new(&bin);
    let e_phoff = usize::from_str_radix(&elf32_phoff(hdr.e_phoff), 16).unwrap();
    let e_phnum = usize::from_str_radix(&elf_phnum(hdr.e_phnum), 16).unwrap();
    let mut phdr_vec = Vec::new();
    for offset in 0..e_phnum {
        phdr_vec.push(Elf32Phdr::new(&bin, e_phoff + 32*offset));
    }

    /* print program header */
    let mut phdr_table = Table::new();
    phdr_table.add_row(row!["Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flags", "Align"]);
    for phdr in phdr_vec.iter() {
        phdr_table.add_row(row![
            phdr_type(phdr.p_type),
            format!("{}{}", "0x".to_string(), phdr32_offset(phdr.p_offset)),
            format!("{}{}", "0x".to_string(), phdr32_vaddr(phdr.p_vaddr)),
            format!("{}{}", "0x".to_string(), phdr32_paddr(phdr.p_paddr)),
            format!("{}{}", "0x".to_string(), phdr32_filesz(phdr.p_filesz)),
            format!("{}{}", "0x".to_string(), phdr32_memsz(phdr.p_memsz)),
            phdr_flags(phdr.p_flags),
            format!("{}{}", "0x".to_string(), phdr32_align(phdr.p_align)),
        ]);
    }
    phdr_table.printstd();
}