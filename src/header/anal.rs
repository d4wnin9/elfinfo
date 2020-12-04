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
// const ELFOSABI_HPUX: u8 = 0x01;
// const ELFOSABI_NETBSD: u8 = 0x02;
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
// const EM_NONE: [u8; 2] = [0x00, 0x00];
// const EM_M32: [u8; 2] = [0x01, 0x00];
// const EM_SPARC: [u8; 2] = [0x02, 0x00];
// const EM_386: [u8; 2] = [0x03, 0x00];
// const EM_68K: [u8; 2] = [0x04, 0x00];
// const EM_88K: [u8; 2] = [0x05, 0x00];
// resrved: [u8; 2] = [0x06, 0x00];  // reserved for future use (was EM_486)
// const EM_860: [u8; 2] = [0x07, 0x00];
const EM_X86_64: [u8; 2] = [0x3e, 0x00];
const EM_AARCH_64: [u8; 2] = [0xb7, 0x00];

/* RWX Flagの管理どうしたら... */
const PF_X: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const PF_W: [u8; 4] = [0x02, 0x00, 0x00, 0x00];
const PF_WX: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
const PF_R: [u8; 4] = [0x04, 0x00, 0x00, 0x00];
const PF_RX: [u8; 4] = [0x05, 0x00, 0x00, 0x00];
const PF_RW: [u8; 4] = [0x06, 0x00, 0x00, 0x00];
const PF_RWX: [u8; 4] = [0x07, 0x00, 0x00, 0x00];

/* Program Type */
const PT_NULL: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
const PT_LOAD: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const PT_DYNAMIC: [u8; 4] = [0x02, 0x00, 0x00, 0x00];
const PT_INTERP: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
const PT_NOTE: [u8; 4] = [0x04, 0x00, 0x00, 0x00];
const PT_SHLIB: [u8; 4] = [0x05, 0x00, 0x00, 0x00];
const PT_PHDR: [u8; 4] = [0x06, 0x00, 0x00, 0x00];
const PT_LOPROC: [u8; 4] = [0x07, 0x00, 0x00, 0x00];
const PT_HIPROC: [u8; 4] = [0x08, 0x00, 0x00, 0x00];
const PT_GNU_EH_FRAME: [u8; 4] = [0x50, 0xe5, 0x74, 0x64];
const PT_GNU_STACK: [u8; 4] = [0x51, 0xe5, 0x74, 0x64];
const PT_GNU_RELRO: [u8; 4] = [0x52, 0xe5, 0x74, 0x64];
const PT_GNU_PROPERTY: [u8; 4] = [0x53, 0xe5, 0x74, 0x64];


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
pub fn elf32_entry(e_entry: Elf32Addr) -> String {
    e_entry.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf64_entry(e_entry: Elf64Addr) -> String {
    e_entry.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf32_phoff(e_phoff: Elf32Addr) -> String {
    e_phoff.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf64_phoff(e_phoff: Elf64Off) -> String {
    e_phoff.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf32_shoff(e_shoff: Elf32Addr) -> String {
    e_shoff.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn elf64_shoff(e_shoff: Elf64Off) -> String {
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


pub fn phdr_type(p_type: Elf64Word) -> String {
    if p_type == PT_NULL {
        "NULL".to_string()
    } else if p_type == PT_LOAD {
        "LOAD".to_string()
    } else if p_type == PT_DYNAMIC {
        "DYNAMIC".to_string()
    } else if p_type == PT_INTERP {
        "INTERP".to_string()
    } else if p_type == PT_NOTE {
        "NOTE".to_string()
    } else if p_type == PT_SHLIB {
        "SHLIB".to_string()
    } else if p_type == PT_PHDR {
        "PHDR".to_string()
    } else if p_type == PT_LOPROC {
        "LOPROC".to_string()
    } else if p_type == PT_HIPROC {
        "HIPROC".to_string()
    } else if p_type == PT_GNU_EH_FRAME {
        "FRAME".to_string()
    } else if p_type == PT_GNU_STACK {
        "GNU_STACK".to_string()
    } else if p_type == PT_GNU_RELRO {
        "GNU_RELRO".to_string()
    } else if p_type == PT_GNU_PROPERTY {
        "GNU_PROPERTY".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn phdr_flags(p_flags: Elf64Word) -> String {
    if p_flags == PF_X {
        "X".to_string()
    } else if p_flags == PF_W {
        "W".to_string()
    } else if p_flags == PF_WX {
        "WX".to_string()
    } else if p_flags == PF_R {
        "R".to_string()
    } else if p_flags == PF_RX {
        "RX".to_string()
    } else if p_flags == PF_RW {
        "RW".to_string()
    } else if p_flags == PF_RWX {
        "RWX".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn phdr32_offset(p_offset: Elf32Off) -> String {
    p_offset.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_offset(p_offset: Elf64Off) -> String {
    p_offset.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr32_vaddr(p_vaddr: Elf32Addr) -> String {
    p_vaddr.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_vaddr(p_vaddr: Elf64Addr) -> String {
    p_vaddr.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr32_paddr(p_paddr: Elf32Addr) -> String {
    p_paddr.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_paddr(p_paddr: Elf64Addr) -> String {
    p_paddr.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr32_filesz(p_filesz: Elf32Word) -> String {
    p_filesz.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_filesz(p_filesz: Elf64Xword) -> String {
    p_filesz.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr32_memsz(p_memsz: Elf32Word) -> String {
    p_memsz.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_memsz(p_memsz: Elf64Xword) -> String {
    p_memsz.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr32_align(p_align: Elf32Word) -> String {
    p_align.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}

pub fn phdr64_align(p_align: Elf64Xword) -> String {
    p_align.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>()
}