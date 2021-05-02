use crate::*;
use u16;


/* elf64 header */
#[derive(Default, Debug, Clone, Copy)]
pub struct Ehdr64 {
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

impl Ehdr64 {
    pub fn new(buf: &[u8]) -> Ehdr64 {
        let mut e_ident = [0; 16];
        for (i, b) in buf[0..16].iter().enumerate() {
            e_ident[i] = *b;
        }
        let e_type = u16::from(buf[16]) + (u16::from(buf[17]) * 1<<8);
        let e_machine = u16::from(buf[18]) + (u16::from(buf[19]) * 1<<8);
        let e_version = u32::from(buf[20]) + (u32::from(buf[21]) * 1<<8) + (u32::from(buf[22]) * 1<<16) + (u32::from(buf[23]) * 1<<24);
        let e_entry = u64::from(buf[24]) + (u64::from(buf[25]) * 1<<8) + (u64::from(buf[26]) * 1<<16) + (u64::from(buf[27]) * 1<<24) + (u64::from(buf[28]) * 1<<32) + (u64::from(buf[29]) * 1<<40) + (u64::from(buf[30]) * 1<<48) + (u64::from(buf[31]) * 1<<56);
        let e_phoff = u64::from(buf[32]) + (u64::from(buf[33]) * 1<<8) + (u64::from(buf[34]) * 1<<16) + (u64::from(buf[35]) * 1<<24) + (u64::from(buf[36]) * 1<<32) + (u64::from(buf[37]) * 1<<40) + (u64::from(buf[38]) * 1<<48) + (u64::from(buf[39]) * 1<<56);
        let e_shoff = u64::from(buf[40]) + (u64::from(buf[41]) * 1<<8) + (u64::from(buf[42]) * 1<<16) + (u64::from(buf[43]) * 1<<24) + (u64::from(buf[44]) * 1<<32) + (u64::from(buf[45]) * 1<<40) + (u64::from(buf[46]) * 1<<48) + (u64::from(buf[47]) * 1<<56);
        let e_flags = u32::from(buf[48]) + (u32::from(buf[49]) * 1<<8) + (u32::from(buf[50]) * 1<<16) + (u32::from(buf[51]) * 1<<24);
        let e_hsize = u16::from(buf[52]) + (u16::from(buf[53]) * 1<<8);
        let e_phsize = u16::from(buf[54]) + (u16::from(buf[55]) * 1<<8);
        let e_phnum = u16::from(buf[56]) + (u16::from(buf[57]) * 1<<8);
        let e_shsize = u16::from(buf[58]) + (u16::from(buf[59]) * 1<<8);
        let e_shnum = u16::from(buf[60]) + (u16::from(buf[61]) * 1<<8);
        let e_shstrndx = u16::from(buf[62]) + (u16::from(buf[63]) * 1<<8);
        Ehdr64 {
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