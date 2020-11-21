use crate::*;

use std::io::{self, Write};


// elf64 header template
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
        let mut e_type: [u8; 2] = [0; 2];
        for (i, b) in bin[16..18].iter().enumerate() {
            e_type[i] = *b;
        }
        let mut e_machine: [u8; 2] = [0; 2];
        for (i, b) in bin[18..20].iter().enumerate() {
            e_machine[i] = *b;
        }
        let mut e_version: [u8; 4] = [0; 4];
        for (i, b) in bin[20..24].iter().enumerate() {
            e_version[i] = *b;
        }
        let mut e_entry: [u8; 8] = [0; 8];
        for (i, b) in bin[24..32].iter().enumerate() {
            e_entry[i] = *b;
        }
        let mut e_phoff: [u8; 8] = [0; 8];
        for (i, b) in bin[32..40].iter().enumerate() {
            e_phoff[i] = *b;
        }
        let mut e_shoff: [u8; 8] = [0; 8];
        for (i, b) in bin[40..48].iter().enumerate() {
            e_shoff[i] = *b;
        }
        let mut e_flags: [u8; 4] = [0; 4];
        for (i, b) in bin[48..52].iter().enumerate() {
            e_flags[i] = *b;
        }
        let mut e_hsize: [u8; 2] = [0; 2];
        for (i, b) in bin[52..54].iter().enumerate() {
            e_hsize[i] = *b;
        }
        let mut e_phsize: [u8; 2] = [0; 2];
        for (i, b) in bin[54..56].iter().enumerate() {
            e_phsize[i] = *b;
        }
        let mut e_phnum: [u8; 2] = [0; 2];
        for (i, b) in bin[56..58].iter().enumerate() {
            e_phnum[i] = *b;
        }
        let mut e_shsize: [u8; 2] = [0; 2];
        for (i, b) in bin[58..60].iter().enumerate() {
            e_shsize[i] = *b;
        }
        let mut e_shnum: [u8; 2] = [0; 2];
        for (i, b) in bin[60..62].iter().enumerate() {
            e_shnum[i] = *b;
        }
        let mut e_shstrndx: [u8; 2] = [0; 2];
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

// elf64 header preview
pub fn show_hdr(hdr: Elf64Hdr) {
    if is_elf(hdr.e_ident) {
        println!("ELF Header:");
    } else {
        println!("Broken ELF Header :(");
    }

    println!("  Magic:   {}", ei_magic(hdr.e_ident));

    println!("  Class:                             {}", ei_class(hdr.e_ident));

    println!("  Data:                              {}", ei_data(hdr.e_ident));

    println!("  Version:                           {}", ei_version(hdr.e_ident));

    println!("  OS/ABI:                            {}", ei_osabi(hdr.e_ident));

    println!("  Type:                              {}", elf_type(hdr.e_type));

    println!("  Machine:                           {}", elf_machine(hdr.e_machine));

    println!("  Entry point address:               0x{}", elf_entry(hdr.e_entry));



    println!("\n\n");

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