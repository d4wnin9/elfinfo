use crate::*;
use super::{Ehdr32, Ehdr64};


#[derive(Debug)]
pub enum Ehdr {
    Ehdr32(Ehdr32),
    Ehdr64(Ehdr64),
}


impl Ehdr {
    pub fn as_32bit(&self) -> Ehdr32 {
        match self {
            Ehdr::Ehdr32(ehdr) => *ehdr,
            _ => unreachable!(),
        }
    }
    pub fn as_64bit(&self) -> Ehdr64 {
        match self {
            Ehdr::Ehdr64(ehdr) => *ehdr,
            _ => unreachable!(),
        }
    }
}


/* EI_MAG */
pub const EI_MAG: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

/* EI_CLASS */
#[derive(Debug)]
pub enum EiClass {
    None,
    Elf32,
    Elf64,
    Num,
}

#[allow(unreachable_patterns)]
impl EiClass {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            Self::Elf32 => 0x01,
            Self::Elf64 => 0x02,
            Self::Num => 0x03,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for EiClass {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Elf32,
            0x02 => Self::Elf64,
            0x03 => Self::Num,
            _ => unreachable!(),
        }
    }
}


/* e_ident[EI_DATA] */
#[derive(Debug)]
pub enum EiData {
    None,
    Lsb,
    Msb,
}

#[allow(unreachable_patterns)]
impl EiData {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            Self::Lsb => 0x01,
            Self::Msb => 0x02,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for EiData {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Lsb,
            0x02 => Self::Msb,
            _ => unreachable!(),
        }
    }
}

/* EI_VERSION */
#[derive(Debug)]
pub enum EiVersion {
    None,
    Current,
    Num,
}

#[allow(unreachable_patterns)]
impl EiVersion {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            Self::Current => 0x01,
            Self::Num => 0x02,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for EiVersion {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Current,
            0x02 => Self::Num,
            _ => unreachable!(),
        }
    }
}


/* e_machine */
#[derive(Debug)]
pub enum EiOsAbi {
    None,
    Hpux,
    NetBsd,
    Linux,
}

#[allow(unreachable_patterns)]
impl EiOsAbi {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            Self::Hpux => 0x01,
            Self::NetBsd => 0x02,
            Self::Linux => 0x03,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for EiOsAbi {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Hpux,
            0x02 => Self::NetBsd,
            0x03 => Self::Linux,
            _ => unreachable!(),
        }
    }
}


/* These constants define the different elf file types */
#[derive(Debug)]
pub enum EType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

#[allow(unreachable_patterns)]
impl EType {
    pub fn to_id(&self) -> Elf64Half {
        match self {
            Self::None => 0x00,
            Self::Rel => 0x01,
            Self::Exec => 0x02,
            Self::Dyn => 0x03,
            Self::Core => 0x04,
            Self::Loos => 0xfe00,
            Self::Hios => 0xfeff,
            Self::Loproc => 0xff00,
            Self::Hiproc => 0xffff,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<Elf64Half> for EType {
    fn from(byte: Elf64Half) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::Rel,
            0x02 => Self::Exec,
            0x03 => Self::Dyn,
            0x04 => Self::Core,
            0xfe00 => Self::Loos,
            0xfeff => Self::Hios,
            0xff00 => Self::Loproc,
            0xffff => Self::Hiproc,
            _ => unreachable!(),
        }
    }
}


/* Machine list */
#[derive(Debug)]
pub enum EMachine {
    None,
    M32,
    Sparc,
    I386,
    M68K,
    M88K,
    I486,
    I860,
    X64,
    AArch64,
}

#[allow(unreachable_patterns)]
impl EMachine {
    pub fn to_id(&self) -> Elf64Half {
        match self {
            Self::None => 0x00,
            Self::M32 => 0x01,
            Self::Sparc => 0x02,
            Self::I386 => 0x03,
            Self::M68K => 0x04,
            Self::M88K => 0x05,
            Self::I486 => 0x06,
            Self::I860 => 0x07,
            Self::X64 => 0x3e,
            Self::AArch64 => 0xb7,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<Elf64Half> for EMachine {
    fn from(byte: Elf64Half) -> Self {
        match byte {
            0x00 => Self::None,
            0x01 => Self::M32,
            0x02 => Self::Sparc,
            0x03 => Self::I386,
            0x04 => Self::M68K,
            0x05 => Self::M88K,
            0x06 => Self::I486,
            0x07 => Self::I860,
            0x3e => Self::X64,
            0xb7 => Self::AArch64,
            _ => unreachable!(),
        }
    }
}

// e_versionは何に必要なのか分からないのでパス

/* p_flags */
#[derive(Debug)]
pub enum PFlags {
    None,
}

#[allow(unreachable_patterns)]
impl PFlags {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for PFlags {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            _ => unreachable!(),
        }
    }
}


/* Program Type */
#[derive(Debug)]
pub enum PType {
    None,
}

#[allow(unreachable_patterns)]
impl PType {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for PType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            _ => unreachable!(),
        }
    }
}


/* sh_type */
#[derive(Debug)]
pub enum ShType {
    None,
}

#[allow(unreachable_patterns)]
impl ShType {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for ShType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            _ => unreachable!(),
        }
    }
}


/* sh_flags */
#[derive(Debug)]
pub enum ShFlags {
    None,
}

#[allow(unreachable_patterns)]
impl ShFlags {
    pub fn to_id(&self) -> u8 {
        match self {
            Self::None => 0x00,
            _ => unreachable!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl From<u8> for ShFlags {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::None,
            _ => unreachable!(),
        }
    }
}