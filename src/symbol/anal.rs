use crate::*;
use std::usize;
use std::vec::Vec;

/* This info is needed when parsing the symbol table */
/*
const STB_LOCAL = 0;
const STB_GLOBAL = 1;
const STB_WEAK = 2;
const STB_LOOS = 10;
const STB_HIOS = 12;
const STB_LOPROC = 13;
const STB_HIPROC = 15;

const STT_NOTYPE = 0;
const STT_OBJECT = 1;
const STT_FUNC = 2;
const STT_SECTION = 3;
const STT_FILE = 4;
const STT_COMMON = 5;
const STT_TLS = 6;
const STT_LOOS = 10;
const STT_HIOS = 12;
const STT_LOPROC = 13;
const STT_HIPROC = 15;

const STV_DEFAULT = 0;
const STV_INTERNAL = 1;
const STV_HIDDEN = 2;
const STV_PROTECTED = 3;
*/

pub fn sym64_name(st_name: Elf64Word, strtab_offset: usize, strtab_size: usize, bin: &[u8]) -> String {
    /* strtabを抜き出す */
    let mut strtab_vec = Vec::new();
    for chr in bin[strtab_offset..strtab_offset+strtab_size].iter() {
        strtab_vec.push(chr);
    }
    /* sh_nameをshstrtableから抜き出してくる */
    let mut st_name_vec = Vec::new();
    for chr in strtab_vec[usize::from_str_radix(&st_name.iter().rev().map(|n| format!("{:02x}", n)).collect::<String>(), 16).unwrap()..].iter() {
        /* amazing magic && */
        if chr == &&0x00 {
            break;
        }
        st_name_vec.push(chr);
    }
    /* sh_name_vecをstringにしてる */
    st_name_vec.iter().map(|&s| **s as char).collect::<String>()
}
/*
sym64_bind: st_info >> 4
sym64_type: st_info & 0xf
sym64_visible: st_other
*/