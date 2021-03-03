use crate::*;
use std::usize;
use std::vec::Vec;

/* もっといい方法でやる

const DT_NULL: usize = 0;
const DT_NEEDED: usize = 1;
const DT_PLTRELSZ: usize = 2;
const DT_PLTGOT: usize = 3;
const DT_HASH: usize = 4;
const DT_STRTAB: usize = 5;
const DT_SYMTAB: usize = 6;
const DT_RELA: usize = 7;
const DT_RELASZ: usize = 8;
const DT_RELAENT: usize = 9;
const DT_STRSZ: usize = 10;
const DT_SYMENT: usize = 11;
const DT_INIT: usize = 12;
const DT_FINI: usize = 13;
const DT_SONAME: usize = 14;
const DT_RPATH: usize = 15;
const DT_SYMBOLIC: usize = 16;
const DT_RELG: usize = 21;
const DT_TEXTREL: usize = 22;
const DT_JMPREL: usize = 23;
const DT_ENCODING: usize = 32;
const OLD_DT_LOOS: [u8; 4] = [0x00, 0x00, 0x00, 0x60];
const DT_LOOS: [u8; 4] = [0x0d, 0x00, 0x00, 0x60];
const DT_HIOS: [u8; 4] = [0x00, 0xf0, 0xff, 0x6f];
const DT_VALRNGLO: [u8; 4] = [0x00, 0xfd, 0xff, 0x6f];
const DT_VALRNGHI: [u8; 4] = [0xff, 0xfd, 0xff, 0x6f];
const DT_ADDRRNGLO: [u8; 4] = [0x00, 0xfe, 0xff, 0x6f];
const DT_ADDRRNGHI: [u8; 4] = [0xff, 0xfe, 0xff, 0x6f];
const DT_VERSYM: [u8; 4] = [0xf0, 0xff, 0xff, 0x6f];
const DT_RELACOUNT: [u8; 4] = [0xf9, 0xff, 0xff, 0x6f];
const DT_RELCOUNT: [u8; 4] = [0xfa, 0xff, 0xff, 0x6f];
const DT_FLAGS_1: [u8; 4] = [0xfb, 0xff, 0xff, 0x6f];
const DT_VERDEF: [u8; 4] = [0xfc, 0xff, 0xff, 0x6f];
const DT_VERDEFNUM: [u8; 4] = [0xfd, 0xff, 0xff, 0x6f];
const DT_VERNEED: [u8; 4] = [0xfe, 0xff, 0xff, 0x6f];
const DT_VERNEEDNUM: [u8; 4] = [0xff, 0xff, 0xff, 0x6f];
const DT_OLD_DT_HIOS: [u8; 4] = [0xff, 0xff, 0xff, 0x6f];
const DT_LOPROC: [u8; 4] = [0x00, 0x00, 0x00, 0x70];
const DT_HIPROC: [u8; 4] = [0xff, 0xff, 0xff, 0x7f];

*/
pub fn dyn64_tag(d_tag: Elf64Sxword) -> String {
    /* DEBUGかどうかだけとりあえず */
    if d_tag == [0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
        "FLAGS".to_string()
    } else if d_tag == [0x15, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
        "DEBUG".to_string()
    } else {
        "???".to_string()
    }
}

pub fn dyn64_un(d_tag: Elf64Sxword, d_un: Elf64Xword) -> String {
    if d_tag == [0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
        if d_un == [0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] {
            "BIND_NOW".to_string()
        } else {
            "???".to_string()
        }
    } else {
        "???".to_string()
    }
}