use zerocopy::FromBytes;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct Header64 {
    pub magic: Magic,
    pub class: Class,
    pub encoding: Encoding,
    pub version: u8,
    pub os_abi: OsAbi,
    pub abi_version: u8,
    pub pad: [u8; 7],
    pub object_type: ObjectType,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct Magic {
    inner: [u8; 4],
}

impl Magic {
    pub fn is_valid(&self) -> bool {
        self.inner == [0x7f, b'E', b'L', b'F']
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct Class {
    inner: u8,
}

impl Class {
    pub const NONE: Self = Self::from_raw(0);
    pub const ELF32: Self = Self::from_raw(1);
    pub const ELF64: Self = Self::from_raw(2);

    pub const fn from_raw(raw: u8) -> Self {
        Self { inner: raw }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct Encoding {
    inner: u8,
}

impl Encoding {
    pub const NONE: Self = Self::from_raw(0);
    pub const ELFDATA2LSB: Self = Self::from_raw(1);
    pub const ELFDATA2MSB: Self = Self::from_raw(2);

    pub const fn from_raw(raw: u8) -> Self {
        Self { inner: raw }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct OsAbi {
    inner: u8,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct ObjectType {
    inner: u16,
}

impl ObjectType {
    pub const NONE: Self = Self::from_raw(0);
    pub const REL: Self = Self::from_raw(1);
    pub const EXEC: Self = Self::from_raw(2);
    pub const DYN: Self = Self::from_raw(3);
    pub const CORE: Self = Self::from_raw(4);
    pub const LOOS: Self = Self::from_raw(0xfe00);
    pub const HIOS: Self = Self::from_raw(0xfeff);
    pub const LOPROC: Self = Self::from_raw(0xff00);
    pub const HIPROC: Self = Self::from_raw(0xffff);

    pub const fn from_raw(raw: u16) -> Self {
        Self { inner: raw }
    }
}
