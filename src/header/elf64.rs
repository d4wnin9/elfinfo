use crate::*;
use std::usize;
use std::vec::Vec;
use prettytable::Table;


/* elf64 header */
pub struct Elf64Hdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_hsize: Elf64Half,
    pub e_phsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

impl Elf64Hdr {
    pub fn new(bin: &[u8]) -> Elf64Hdr {
        let mut e_ident: [u8; 16] = [0; 16];
        for (i, b) in bin[0..16].iter().enumerate() {
            e_ident[i] = *b;
        }
        let mut e_type: Elf64Half = [0; 2];
        for (i, b) in bin[16..18].iter().enumerate() {
            e_type[i] = *b;
        }
        let mut e_machine: Elf64Half = [0; 2];
        for (i, b) in bin[18..20].iter().enumerate() {
            e_machine[i] = *b;
        }
        let mut e_version: Elf64Word = [0; 4];
        for (i, b) in bin[20..24].iter().enumerate() {
            e_version[i] = *b;
        }
        let mut e_entry: Elf64Addr = [0; 8];
        for (i, b) in bin[24..32].iter().enumerate() {
            e_entry[i] = *b;
        }
        let mut e_phoff: Elf64Off = [0; 8];
        for (i, b) in bin[32..40].iter().enumerate() {
            e_phoff[i] = *b;
        }
        let mut e_shoff: Elf64Off = [0; 8];
        for (i, b) in bin[40..48].iter().enumerate() {
            e_shoff[i] = *b;
        }
        let mut e_flags: Elf64Word = [0; 4];
        for (i, b) in bin[48..52].iter().enumerate() {
            e_flags[i] = *b;
        }
        let mut e_hsize: Elf64Half = [0; 2];
        for (i, b) in bin[52..54].iter().enumerate() {
            e_hsize[i] = *b;
        }
        let mut e_phsize: Elf64Half = [0; 2];
        for (i, b) in bin[54..56].iter().enumerate() {
            e_phsize[i] = *b;
        }
        let mut e_phnum: Elf64Half = [0; 2];
        for (i, b) in bin[56..58].iter().enumerate() {
            e_phnum[i] = *b;
        }
        let mut e_shsize: Elf64Half = [0; 2];
        for (i, b) in bin[58..60].iter().enumerate() {
            e_shsize[i] = *b;
        }
        let mut e_shnum: Elf64Half = [0; 2];
        for (i, b) in bin[60..62].iter().enumerate() {
            e_shnum[i] = *b;
        }
        let mut e_shstrndx: Elf64Half = [0; 2];
        for (i, b) in bin[62..64].iter().enumerate() {
            e_shstrndx[i] = *b;
        }
        Elf64Hdr {
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

/* elf64 program header */
pub struct Elf64Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

impl Elf64Phdr {
    pub fn new(bin: &[u8], e_phoff: usize) -> Elf64Phdr {
        let mut p_type: Elf64Word = [0; 4];
        for (i, b) in bin[e_phoff..e_phoff+4].iter().enumerate() {
            p_type[i] = *b;
        }
        let mut p_flags: Elf64Word = [0; 4];
        for (i, b) in bin[e_phoff+4..e_phoff+8].iter().enumerate() {
            p_flags[i] = *b;
        }
        let mut p_offset: Elf64Off = [0; 8];
        for (i, b) in bin[e_phoff+8..e_phoff+16].iter().enumerate() {
            p_offset[i] = *b;
        }
        let mut p_vaddr: Elf64Addr = [0; 8];
        for (i, b) in bin[e_phoff+16..e_phoff+24].iter().enumerate() {
            p_vaddr[i] = *b;
        }
        let mut p_paddr: Elf64Addr = [0; 8];
        for (i, b) in bin[e_phoff+24..e_phoff+32].iter().enumerate() {
            p_paddr[i] = *b;
        }
        let mut p_filesz: Elf64Xword = [0; 8];
        for (i, b) in bin[e_phoff+32..e_phoff+40].iter().enumerate() {
            p_filesz[i] = *b;
        }
        let mut p_memsz: Elf64Xword = [0; 8];
        for (i, b) in bin[e_phoff+40..e_phoff+48].iter().enumerate() {
            p_memsz[i] = *b;
        }
        let mut p_align: Elf64Xword = [0; 8];
        for (i, b) in bin[e_phoff+48..e_phoff+56].iter().enumerate() {
            p_align[i] = *b;
        }
        Elf64Phdr {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        }
    }
}

pub struct Elf64Shdr {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
}

impl Elf64Shdr {
    pub fn new(bin: &[u8], e_shoff: usize) -> Elf64Shdr {
        let mut sh_name: Elf64Word = [0; 4];
        for (i, b) in bin[e_shoff..e_shoff+4].iter().enumerate() {
            sh_name[i] = *b;
        }
        let mut sh_type: Elf64Word = [0; 4];
        for (i, b) in bin[e_shoff+4..e_shoff+8].iter().enumerate() {
            sh_type[i] = *b;
        }
        let mut sh_flags: Elf64Xword = [0; 8];
        for (i, b) in bin[e_shoff+8..e_shoff+16].iter().enumerate() {
            sh_flags[i] = *b;
        }
        let mut sh_addr: Elf64Addr = [0; 8];
        for (i, b) in bin[e_shoff+16..e_shoff+24].iter().enumerate() {
            sh_addr[i] = *b;
        }
        let mut sh_offset: Elf64Off = [0; 8];
        for (i, b) in bin[e_shoff+24..e_shoff+32].iter().enumerate() {
            sh_offset[i] = *b;
        }
        let mut sh_size: Elf64Xword = [0; 8];
        for (i, b) in bin[e_shoff+32..e_shoff+40].iter().enumerate() {
            sh_size[i] = *b;
        }
        let mut sh_link: Elf64Word = [0; 4];
        for (i, b) in bin[e_shoff+40..e_shoff+44].iter().enumerate() {
            sh_link[i] = *b;
        }
        let mut sh_info: Elf64Word = [0; 4];
        for (i, b) in bin[e_shoff+44..e_shoff+48].iter().enumerate() {
            sh_info[i] = *b;
        }
        let mut sh_addralign: Elf64Xword = [0; 8];
        for (i, b) in bin[e_shoff+48..e_shoff+56].iter().enumerate() {
            sh_addralign[i] = *b;
        }
        let mut sh_entsize: Elf64Xword = [0; 8];
        for (i, b) in bin[e_shoff+56..e_shoff+64].iter().enumerate() {
            sh_entsize[i] = *b;
        }
        Elf64Shdr {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        }
    }
}


/* elf64 header preview */
pub fn print_elf64_hdr(hdr: Elf64Hdr) {
    println!("ELF Header:");
    println!("  Magic:   {}", ei_magic(hdr.e_ident));
    println!("  Class:                             {}", ei_class(hdr.e_ident));
    println!("  Data:                              {}", ei_data(hdr.e_ident));
    println!("  Version:                           {}", ei_version(hdr.e_ident));
    println!("  OS/ABI:                            {}", ei_osabi(hdr.e_ident));
    println!("  Type:                              {}", elf_type(hdr.e_type));
    println!("  Machine:                           {}", elf_machine(hdr.e_machine));
    println!("  Entry point address:               0x{}", elf64_entry(hdr.e_entry));
    println!("  Start of program headers:          0x{} (bytes into file)", elf64_phoff(hdr.e_phoff));
    println!("  Start of section headers:          0x{} (bytes into file)", elf64_shoff(hdr.e_shoff));
    println!("  Flags:                             0x{}", elf_flags(hdr.e_flags));
    println!("  Size of this header:               0x{} (bytes)", elf_hsize(hdr.e_hsize));
    println!("  Size of program headers:           0x{} (bytes)", elf_phsize(hdr.e_phsize));
    println!("  Number of program headers:         0x{}", elf_phnum(hdr.e_phnum));
    println!("  Size of section headers:           0x{} (bytes)", elf_shsize(hdr.e_shsize));
    println!("  Number of section headers:         0x{}", elf_shnum(hdr.e_shnum));
    println!("  Section header string table index: 0x{}", elf_shstrndx(hdr.e_shstrndx));
}

pub fn print_elf64_phdr(hdr: Elf64Hdr, bin: &[u8]) {
    let e_phoff = usize::from_str_radix(&elf64_phoff(hdr.e_phoff), 16).unwrap();
    let e_phnum = usize::from_str_radix(&elf_phnum(hdr.e_phnum), 16).unwrap();
    let mut phdr_vec = Vec::new();
    for offset in 0..e_phnum {
        phdr_vec.push(Elf64Phdr::new(&bin, e_phoff + 56*offset));
    }

    /* print program header */
    let mut phdr_table = Table::new();
    phdr_table.add_row(row!["Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flags", "Align"]);
    for phdr in phdr_vec.iter() {
        phdr_table.add_row(row![
            phdr_type(phdr.p_type),
            format!("{}{}", "0x".to_string(), phdr64_offset(phdr.p_offset)),
            format!("{}{}", "0x".to_string(), phdr64_vaddr(phdr.p_vaddr)),
            format!("{}{}", "0x".to_string(), phdr64_paddr(phdr.p_paddr)),
            format!("{}{}", "0x".to_string(), phdr64_filesz(phdr.p_filesz)),
            format!("{}{}", "0x".to_string(), phdr64_memsz(phdr.p_memsz)),
            phdr_flags(phdr.p_flags),
            format!("{}{}", "0x".to_string(), phdr64_align(phdr.p_align)),
        ]);
    }
    phdr_table.printstd();
}

pub fn print_elf64_shdr(hdr: Elf64Hdr, bin: &[u8]) {
    let e_shoff = usize::from_str_radix(&elf64_shoff(hdr.e_shoff), 16).unwrap();
    let e_shnum = usize::from_str_radix(&elf_shnum(hdr.e_shnum), 16).unwrap();
    let mut shdr_vec = Vec::new();
    for offset in 0..e_shnum {
        shdr_vec.push(Elf64Shdr::new(&bin, e_shoff + 64*offset));
    }


    /* print section header */
    let mut shdr_table = Table::new();
    shdr_table.add_row(row!["Name", "Type", "Address", "Offset", "Size", "EntSize", "Flags", "Link", "Info", "Align"]);
    for shdr in shdr_vec.iter() {
        shdr_table.add_row(row![
            format!("{}", shdr64_name(shdr.sh_name, &shdr_vec, hdr.e_shstrndx, &bin)),
            shdr64_type(shdr.sh_type),
            format!("{}{}", "0x", shdr64_addr(shdr.sh_addr)),
            format!("{}{}", "0x", shdr64_offset(shdr.sh_offset)),
            format!("{}{}", "0x", shdr64_size(shdr.sh_size)),
            format!("{}{}", "0x", shdr64_entsize(shdr.sh_entsize)),
            shdr64_flags(shdr.sh_flags),
            format!("{}{}", "0x", shdr64_link(shdr.sh_link)),
            format!("{}{}", "0x", shdr64_info(shdr.sh_info)),
            format!("{}{}", "0x", shdr64_addralign(shdr.sh_addralign)),
        ]);
    }
    shdr_table.printstd();
}