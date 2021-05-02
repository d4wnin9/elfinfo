use crate::*;


/* elf32 header */
#[derive(Default, Debug, Clone, Copy)]
pub struct Ehdr32 {
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

impl Ehdr32 {
    pub fn new(buf: &[u8]) -> Ehdr32 {
        let mut e_ident = [0; 16];
        for (i, b) in buf[0..16].iter().enumerate() {
            e_ident[i] = *b;
        }
        let e_type = u16::from(buf[16]) + (u16::from(buf[17]) * 1<<8);
        let e_machine = u16::from(buf[18]) + (u16::from(buf[19]) * 1<<8);
        let e_version = u32::from(buf[20]) + (u32::from(buf[21]) * 1<<8) + (u32::from(buf[22]) * 1<<16) + (u32::from(buf[23]) * 1<<24);
        let e_entry = u32::from(buf[24]) + (u32::from(buf[25]) * 1<<8) + (u32::from(buf[26]) * 1<<16) + (u32::from(buf[27]) * 1<<24);
        let e_phoff = u32::from(buf[28]) + (u32::from(buf[29]) * 1<<8) + (u32::from(buf[30]) * 1<<16) + (u32::from(buf[31]) * 1<<24);
        let e_shoff = u32::from(buf[32]) + (u32::from(buf[33]) * 1<<8) + (u32::from(buf[34]) * 1<<16) + (u32::from(buf[35]) * 1<<24);
        let e_flags = u32::from(buf[36]) + (u32::from(buf[37]) * 1<<8) + (u32::from(buf[38]) * 1<<16) + (u32::from(buf[39]) * 1<<24);
        let e_hsize = u16::from(buf[40]) + (u16::from(buf[41]) * 1<<8);
        let e_phsize = u16::from(buf[42]) + (u16::from(buf[43]) * 1<<8);
        let e_phnum = u16::from(buf[44]) + (u16::from(buf[45]) * 1<<8);
        let e_shsize = u16::from(buf[46]) + (u16::from(buf[47]) * 1<<8);
        let e_shnum = u16::from(buf[48]) + (u16::from(buf[49]) * 1<<8);
        let e_shstrndx = u16::from(buf[50]) + (u16::from(buf[51]) * 1<<8);
        Ehdr32 {
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