use crate::*;
use std::usize;
use std::vec::Vec;
use prettytable::Table;


pub fn checksec64(bin: &[u8]) {
    let mut relro = 0;
    let mut ssp = 0;
    let mut nx = 0;
    let mut pie = 0;


    let hdr = Elf64Hdr::new(&bin);

    if hdr.e_type == [0x02, 0x00] {
        pie += 1;
    } else if hdr.e_type == [0x03, 0x00] {
        pie += 2;
    } else if hdr.e_type == [0x01, 0x00] {
        pie += 4;
    } else {
        pie += 0;
    }


    let e_phoff = usize::from_str_radix(&elf64_phoff(hdr.e_phoff), 16).unwrap();
    let e_phnum = usize::from_str_radix(&elf_phnum(hdr.e_phnum), 16).unwrap();
    let mut phdr_vec = Vec::new();
    for offset in 0..e_phnum {
        phdr_vec.push(Elf64Phdr::new(&bin, e_phoff + 56*offset));
    }

    for phdr in phdr_vec.iter() {
        if phdr.p_type == [0x52, 0xe5, 0x74, 0x64] {
            relro += 1;
        } else {
            relro += 0;
        }

        if phdr.p_type == [0x51, 0xe5, 0x74, 0x64] {
            if phdr.p_flags == [0x07, 0x00, 0x00, 0x00] {
                nx += 0;
            } else {
                nx += 1;
            }
        } else {
            nx += 0
        }
    }


    let e_shoff = usize::from_str_radix(&elf64_shoff(hdr.e_shoff), 16).unwrap();
    let e_shnum = usize::from_str_radix(&elf_shnum(hdr.e_shnum), 16).unwrap();
    let mut shdr_vec = Vec::new();
    for offset in 0..e_shnum {
        shdr_vec.push(Elf64Shdr::new(&bin, e_shoff + 64*offset));
    }
    let mut symtab_vec = Vec::new();
    let mut strtab_offset = 0;
    let mut strtab_size = 0;
    for shdr_offset in 0..e_shnum {
        let shdr = Elf64Shdr::new(&bin, e_shoff + 64*shdr_offset);
        if shdr64_type(shdr.sh_type) == "SYMTAB" {
            /* SYMTABへのoffsetとsize */
            let sh_offset = usize::from_str_radix(&shdr64_size(shdr.sh_offset), 16).unwrap();
            /* symbolの数 */
            let sym_num = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap() / 24;
            for sym_offset in 0..sym_num {
                symtab_vec.push(Elf64Sym::new(&bin, sh_offset + 24*sym_offset));
            }
        } else if shdr64_name(shdr.sh_name, &shdr_vec, hdr.e_shstrndx, &bin) == ".strtab" {
            /* strtabに名前があるから範囲を確認 */
            strtab_offset = usize::from_str_radix(&shdr64_offset(shdr.sh_offset), 16).unwrap();
            strtab_size = usize::from_str_radix(&shdr64_size(shdr.sh_size), 16).unwrap();
        } else {
            continue;
        }
    }

    for symtab in symtab_vec.iter() {
        let st_name = sym64_name(symtab.st_name, strtab_offset, strtab_size, &bin);
        if st_name.contains("__stack_chk_fail") {
            ssp = 1;
        } else if st_name.contains("__intel_security_cookie") {
            ssp = 1;
        } else {
            ssp += 0;
        }
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

    for dynamic in dyn_vec.iter() {
        if relro == 1 {
            if dynamic.d_tag == [0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] && dynamic.d_un == [0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
                relro += 1;
            } else {
                relro += 0;
            }
        } else {
            relro += 0;
        }

        if pie == 2 {
            if dynamic.d_tag == [0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
                pie += 1;
            } else {
                pie += 0;
            }
        } else {
            pie += 0;
        }
    }


    println!("RELRO: {}\nCANARY: {}\nNX: {}\nPIE: {}",
        match relro {
            0 => "\x1b[31mNo RELRO\x1b[0m",
            1 => "\x1b[33mPartial RELRO\x1b[0m",
            2 => "\x1b[32mFull RELRO\x1b[0m",
            _ => "\x1b[43m\x1b[31mError\x1b[0m",
        },
        match ssp {
            0 => "\x1b[31mNo canary found\x1b[0m",
            1 => "\x1b[32mCanary found\x1b[0m",
            _ => "\x1b[43m\x1b[31mError\x1b[0m",
        },
        match nx {
            0 => "\x1b[31mNX disabled\x1b[0m",
            1 => "\x1b[32mNX enabled\x1b[0m",
            _ => "\x1b[43m\x1b[31mError\x1b[0m",
        },
        match pie {
            0 => "\x1b[43m\x1b[31mNot an ELF file\x1b[0m",
            1 => "\x1b[31mNo PIE\x1b[0m",
            2 => "\x1b[34mDSO\x1b[0m",
            3 => "\x1b[32mPIE enabled\x1b[0m",
            4 => "\x1b[35mREL\x1b[0m",
            _ => "\x1b[43m\x1b[31mError\x1b[0m",
        }
    );
}
