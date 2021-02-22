use crate::*;
use std::usize;
use std::vec::Vec;
use prettytable::Table;

pub struct Elf64Dyn {
    pub d_tag: Elf64Sxword,
    pub d_un: Elf64Xword,  // Addrはどうなったんや？
}

impl Elf64Dyn {
    pub fn new(bin: &[u8], offset: usize) -> Elf64Dyn {
        let mut d_tag: Elf64Sxword = [0; 8];
        for (i, b) in bin[offset..offset+8].iter().enumerate() {
            d_tag[i] = *b;
        }
        let mut d_un: Elf64Xword = [0; 8];
        for (i, b) in bin[offset+8..offset+16].iter().enumerate() {
            d_un[i] = *b;
        }
        Elf64Dyn {
            d_tag,
            d_un,
        }
    }
}


pub fn print_dyn64(bin: &[u8]) {
    /* shdrのとこからコピー */
    let hdr = Elf64Hdr::new(&bin);
    let e_shoff = usize::from_str_radix(&elf64_shoff(hdr.e_shoff), 16).unwrap();
    let e_shnum = usize::from_str_radix(&elf_shnum(hdr.e_shnum), 16).unwrap();
    let mut shdr_vec = Vec::new();
    for offset in 0..e_shnum {
        shdr_vec.push(Elf64Shdr::new(&bin, e_shoff + 64*offset));
    }

    let mut dyn_offset = 0;
    let mut dyn_size = 0;

    for shdr_offset in 0..e_shnum {
        let shdr = Elf64Shdr::new(&bin, e_shoff + 64*shdr_offset);
        if shdr64_name(shdr.sh_name, &shdr_vec, hdr.e_shstrndx, &bin) == ".dynamic" {
            /* .dynamicの場所と大きさ */
            dyn_offset = usize::from_str_radix(&shdr64_offset(shdr.sh_offset), 16).unwrap();
            dyn_size = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap();
        } else {
            continue;
        }
    }

    let mut dyn_vec = Vec::new();
    /* dyn_size/16 でdynamicの個数を求めてる */
    for offset in 0..(dyn_size/16) {
        dyn_vec.push(Elf64Dyn::new(&bin, dyn_offset + 16*offset));
    }

    println!("Dynamic section at offset 0x{:x} contains {} entries:", dyn_offset, dyn_size/16);
    println!("Type    Name/Value");
    for dynamic in dyn_vec.iter() {
        println!("{}    {:?}", dyn64_tag(dynamic.d_tag), dynamic.d_un);
    }
}