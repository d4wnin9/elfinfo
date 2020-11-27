use crate::*;


// elf32 header
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

// elf32 program header
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



// elf32 header preview
pub fn print_elf32_hdr(hdr: Elf32Hdr) {
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