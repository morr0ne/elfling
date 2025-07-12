use zerocopy::FromBytes;

use crate::elf_enum;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
#[doc(alias = "Elf64_Phdr")]
pub struct ProgramHeader64 {
    pub p_type: ProgramType,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

elf_enum! {
    pub struct ProgramType(u32) {
        ///
        NULL = 0, "",
        ///
        LOAD = 1, "",
        ///
        DYNAMIC = 2, "",
        ///
        INTERP = 3, "",
        ///
        NOTE = 4, "",
        ///
        SHLIB = 5, "",
        ///
        PHDR = 6, "",
        ///
        TLS = 7, "",
        ///
        LOOS = 0x60000000, "",
        ///
        HIOS = 0x6fffffff, "",
        ///
        LOPROC = 0x70000000, "",
        ///
        HIPROC = 0x7fffffff, "",
    }
}
