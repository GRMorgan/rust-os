use core::default::Default;

#[derive(PartialEq, Debug)]
pub enum ElfPhysicalType {
    ElfPhysicalTypeNull,
    ElfPhysicalTypeLoad,
    ElfPhysicalTypeDynamic,
    ElfPhysicalTypeInterp,
    ElfPhysicalTypeNote,
    ElfPhysicalTypeShlib,
    ElfPhysicalTypePhdr,
    ElfPhysicalTypeTls,
    ElfPhysicalTypeNum,
    ElfPhysicalTypeLoOs,
    ElfPhysicalTypeGnuEhFrame,
    ElfPhysicalTypeGnuStack,
    ElfPhysicalTypeGnuRelRO,
    ElfPhysicalTypeGnuProperty,
    ElfPhysicalTypeSunWBss,
    ElfPhysicalTypeSunWStack,
    ElfPhysicalTypeHiSunW,
    ElfPhysicalTypeLoProc,
    ElfPhysicalTypeHiProc,
}

#[repr(C)]
pub struct ElfPhysicalHeader64 {
    pub _p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl ElfPhysicalHeader64 {
    pub fn p_type(&self) -> ElfPhysicalType {
        match self._p_type {
            1 => ElfPhysicalType::ElfPhysicalTypeLoad,
            2 => ElfPhysicalType::ElfPhysicalTypeDynamic,
            3 => ElfPhysicalType::ElfPhysicalTypeInterp,
            4 => ElfPhysicalType::ElfPhysicalTypeNote,
            5 => ElfPhysicalType::ElfPhysicalTypeShlib,
            6 => ElfPhysicalType::ElfPhysicalTypePhdr,
            7 => ElfPhysicalType::ElfPhysicalTypeTls,
            8 => ElfPhysicalType::ElfPhysicalTypeNum,
            0x60000000 => ElfPhysicalType::ElfPhysicalTypeLoOs,
            0x6474e550 => ElfPhysicalType::ElfPhysicalTypeGnuEhFrame,
            0x6474e551 => ElfPhysicalType::ElfPhysicalTypeGnuStack,
            0x6474e552 => ElfPhysicalType::ElfPhysicalTypeGnuRelRO,
            0x6474e553 => ElfPhysicalType::ElfPhysicalTypeGnuProperty,
            0x6ffffffa => ElfPhysicalType::ElfPhysicalTypeSunWBss,
            0x6ffffffb => ElfPhysicalType::ElfPhysicalTypeSunWStack,
            0x6fffffff => ElfPhysicalType::ElfPhysicalTypeHiSunW,
            0x70000000 => ElfPhysicalType::ElfPhysicalTypeLoProc,
            0x7fffffff => ElfPhysicalType::ElfPhysicalTypeHiProc,
            _ => ElfPhysicalType::ElfPhysicalTypeNull,
        }
    }
}


impl Default for ElfPhysicalHeader64 {
    fn default() -> ElfPhysicalHeader64 {
        ElfPhysicalHeader64 {
            _p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
}
