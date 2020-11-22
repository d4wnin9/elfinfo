use crate::*;


/* e_ident[] indexes */
const EI_MAG0: usize = 0;
// const EI_MAG1: usize = 1;
// const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
// const EI_PAD: usize = 8;

/* EI_MAG */
const ELFMAG0: u8 = 0x7f;
const ELFMAG1: u8 = 0x45;
const ELFMAG2: u8 = 0x4c;
const ELFMAG3: u8 = 0x46;

/* EI_CLASS */
const ELFCLASSNONE: u8 = 0x00;
const ELFCLASS32: u8 = 0x01;
const ELFCLASS64: u8 = 0x02;
// const ELFCLASSNUM: u8 = 0x03;

/* e_ident[EI_DATA] */
const ELFDATANONE: u8 = 0x00;
const ELFDATA2LSB: u8 = 0x01;
const ELFDATA2MSB: u8 = 0x02;

/* e_version, EI_VERSION */
const EV_NONE: u8 = 0x00;
const EV_CURRENT: u8 = 0x01;
const EV_NUM: u8 = 0x02;

/* e_machine */
const ELFOSABI_NONE: u8 = 0x00;
const ELFOSABI_LINUX: u8 = 0x03;

/* These constants define the different elf file types */
const ET_NONE: [u8; 2] = [0x00, 0x00];
const ET_REL: [u8; 2] = [0x01, 0x00];
const ET_EXEC: [u8; 2] = [0x02, 0x00];
const ET_DYN: [u8; 2] = [0x03, 0x00];
const ET_CORE: [u8; 2] = [0x04, 0x00];
const ET_LOOS: [u8; 2] = [0x00, 0xfe];
const ET_HIOS: [u8; 2] = [0xff, 0xfe];
const ET_LOPROC: [u8; 2] = [0x00, 0xff];
const ET_HIPROC: [u8; 2] = [0xff, 0xff];

/* Machine list */
const EM_X86_64: [u8; 2] = [0x3e, 0x00];
const EM_AARCH_64: [u8; 2] = [0xb7, 0x00];


pub fn is_elf(e_ident: [u8; 16]) -> bool {
    e_ident[EI_MAG0..EI_MAG3+1] == [ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3]
}

pub fn ei_magic(e_ident: [u8; 16]) -> String {
    let magic: String = e_ident.iter().map(|n| format!("{:02x} ", n)).collect::<String>();
    magic.trim().to_string()
}

pub fn ei_class(e_ident: [u8; 16]) -> String {
    if e_ident[EI_CLASS] == ELFCLASSNONE {
        "?".to_string()
    } else if e_ident[EI_CLASS] == ELFCLASS32 {
        "ELF32".to_string()
    } else if e_ident[EI_CLASS] == ELFCLASS64 {
        "ELF64".to_string()
    } else {
        "Wrong format".to_string()
    }
}

pub fn ei_data(e_ident: [u8; 16]) -> String {
    if e_ident[EI_DATA] == ELFDATANONE {
        "?".to_string()
    } else if e_ident[EI_DATA] == ELFDATA2LSB {
        "little endian".to_string()
    } else if e_ident[EI_DATA] == ELFDATA2MSB {
        "big endian".to_string()
    } else {
        "Wrong format".to_string()
    }
}

pub fn ei_version(e_ident: [u8; 16]) -> String {
    if e_ident[EI_VERSION] == EV_NONE {
        "?".to_string()
    } else if e_ident[EI_VERSION] == EV_CURRENT {
        "1 (current)".to_string()
    } else if e_ident[EI_VERSION] == EV_NUM {
        "2 (num?)".to_string()
    } else {
        "Wrong format".to_string()
    }
}

pub fn ei_osabi(e_ident: [u8; 16]) -> String {
    if e_ident[EI_OSABI] == ELFOSABI_NONE {
        "No extensions or unspecified".to_string()
    } else if e_ident[EI_OSABI] == ELFOSABI_LINUX {
        "Linux".to_string()
    } else {
        "I don't know".to_string()
    }
}

pub fn elf_type(e_type: Elf64Half) -> String {
    if e_type == ET_NONE {
        "NONE (No file type)".to_string()
    } else if e_type == ET_REL {
        "REL (Relocatable file)".to_string()
    } else if e_type == ET_EXEC {
        "EXEC (Executable file)".to_string()
    } else if e_type == ET_DYN {
        "DYN (Shared object file)".to_string()
    } else if e_type == ET_CORE {
        "CORE (Core file)".to_string()
    } else if e_type == ET_LOOS {
        "LOOS (Operating system-specific)".to_string()
    } else if e_type == ET_HIOS {
        "HIOS (Operating system-specific)".to_string()
    } else if e_type == ET_LOPROC {
        "LOPROC (Processor-specific)".to_string()
    } else if e_type == ET_HIPROC {
        "HIPROC (Processor-specific)".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn elf_machine(e_machine: Elf64Half) -> String {
    if e_machine == EM_X86_64 {
        "x86-64 architecture".to_string()
    } else if e_machine == EM_AARCH_64 {
        "Aarch64".to_string()
    } else {
        "I don't know".to_string()
    }
}
/*
pub fn elf_version(e_version: Elf64Word) {

}
*/

pub fn elf_entry(e_entry: Elf64Addr) -> String {
    e_entry.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}


pub fn elf_phoff(e_phoff: Elf64Off) -> String {
    e_phoff.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_shoff(e_shoff: Elf64Off) -> String {
    e_shoff.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_flags(e_flags: Elf64Word) -> String {
    e_flags.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_hsize(e_hsize: Elf64Half) -> String {
    e_hsize.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_phsize(e_phsize: Elf64Half) -> String {
    e_phsize.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_phnum(e_phnum: Elf64Half) -> String {
    e_phnum.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_shsize(e_shsize: Elf64Half) -> String {
    e_shsize.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_shnum(e_shnum: Elf64Half) -> String {
    e_shnum.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf_shstrndx(e_shstrndx: Elf64Half) -> String {
    e_shstrndx.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}