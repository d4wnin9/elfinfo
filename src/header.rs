// elf64 header template
pub struct Elf64Hdr {
    pub e_ident: [u8; 16],
    pub e_type: [u8; 2],
    pub e_machine: [u8; 2],
    pub e_version: [u8; 4],
    pub e_entry: [u8; 8],
    pub e_phoff: [u8; 8],
    pub e_shoff: [u8; 8],
    pub e_flags: [u8; 4],
    pub e_hsize: [u8; 2],
    pub e_phsize: [u8; 2],
    pub e_phnum: [u8; 2],
    pub e_shsize: [u8; 2],
    pub e_shnum: [u8; 2],
    pub e_shstrndx: [u8; 2],
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