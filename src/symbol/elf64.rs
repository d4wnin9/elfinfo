use crate::*;
use std::usize;
use std::vec::Vec;
use prettytable::Table;


pub struct Elf64Sym {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Half,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}

impl Elf64Sym {
    pub fn new(bin: &[u8], offset: usize) -> Elf64Sym {
        let mut st_name: Elf64Word = [0; 4];
        for (i, b) in bin[offset..offset+4].iter().enumerate() {
            st_name[i] = *b;
        }
        let st_info = bin[offset+4];
        let st_other = bin[offset+5];
        let mut st_shndx: Elf64Half = [0; 2];
        for (i, b) in bin[offset+6..offset+8].iter().enumerate() {
            st_shndx[i] = *b;
        }
        let mut st_value: Elf64Addr = [0; 8];
        for (i, b) in bin[offset+8..offset+16].iter().enumerate() {
            st_value[i] = *b;
        }
        let mut st_size: Elf64Xword = [0; 8];
        for (i, b) in bin[offset+16..offset+24].iter().enumerate() {
            st_size[i] = *b;
        }
        Elf64Sym {
            st_name,
            st_info,
            st_other,
            st_shndx,
            st_value,
            st_size,
        }
    }
}


pub fn print_sym64(bin: &[u8]) {
    /* shdrのとこからコピー */
    let hdr = Elf64Hdr::new(&bin);
    let e_shoff = usize::from_str_radix(&elf64_shoff(hdr.e_shoff), 16).unwrap();
    let e_shnum = usize::from_str_radix(&elf_shnum(hdr.e_shnum), 16).unwrap();
    let mut shdr_vec = Vec::new();
    for offset in 0..e_shnum {
        shdr_vec.push(Elf64Shdr::new(&bin, e_shoff + 64*offset));
    }

    let mut dynsym_vec = Vec::new();
    let mut dynstr_offset = 0;
    let mut dynstr_size = 0;
    let mut symtab_vec = Vec::new();
    let mut strtab_offset = 0;
    let mut strtab_size = 0;

    for shdr_offset in 0..e_shnum {
        let shdr = Elf64Shdr::new(&bin, e_shoff + 64*shdr_offset);
        if shdr64_type(shdr.sh_type) == "DYNSYM" {
            /* DYNSYMへのoffsetとsize */
            let sh_offset = usize::from_str_radix(&shdr64_size(shdr.sh_offset), 16).unwrap();
            /* symbolの数 */
            let sym_num = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap() / 24;
            for sym_offset in 0..sym_num {
                dynsym_vec.push(Elf64Sym::new(&bin, sh_offset + 24*sym_offset));
            }
        } else if shdr64_type(shdr.sh_type) == "SYMTAB" {
            /* SYMTABへのoffsetとsize */
            let sh_offset = usize::from_str_radix(&shdr64_size(shdr.sh_offset), 16).unwrap();
            /* symbolの数 */
            let sym_num = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap() / 24;
            for sym_offset in 0..sym_num {
                symtab_vec.push(Elf64Sym::new(&bin, sh_offset + 24*sym_offset));
            }
        } else if shdr64_name(shdr.sh_name, &shdr_vec, hdr.e_shstrndx, &bin) == ".dynstr" {
            /* dynstrに名前があるから範囲を確認 */
            dynstr_offset = usize::from_str_radix(&shdr64_offset(shdr.sh_offset), 16).unwrap();
            dynstr_size = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap();
        } else if shdr64_name(shdr.sh_name, &shdr_vec, hdr.e_shstrndx, &bin) == ".strtab" {
            /* strtabに名前があるから範囲を確認 */
            strtab_offset = usize::from_str_radix(&shdr64_offset(shdr.sh_offset), 16).unwrap();
            strtab_size = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap();
        } else {
            continue;
        }
    }
    println!("{}\n{}", strtab_offset, strtab_size);
    println!(".dynsym");
    for dynsym in dynsym_vec.iter() {
        println!("{}", sym64_name(dynsym.st_name, dynstr_offset, dynstr_size, &bin));
        println!("{:?}", dynsym.st_info);
        println!("{:?}", dynsym.st_other);
        println!("{:?}", dynsym.st_shndx);
        println!("{:?}", dynsym.st_value);
        println!("{:?}", dynsym.st_size);
        println!("\n");
    }

    println!(".symtab");
    for symtab in symtab_vec.iter() {
        println!("{}", sym64_name(symtab.st_name, strtab_offset, strtab_size, &bin));
        println!("{:?}", symtab.st_info);
        println!("{:?}", symtab.st_other);
        println!("{:?}", symtab.st_shndx);
        println!("{:?}", symtab.st_value);
        println!("{:?}", symtab.st_size);
        println!("\n");
    }
}