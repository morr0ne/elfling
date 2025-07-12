use zerocopy::FromBytes;

use crate::elf_enum;

/// 64-bit ELF header structure (Elf64_Ehdr)
///
/// This structure represents the ELF file header for 64-bit object files.
/// It provides essential information about the file format, target architecture,
/// and layout of the ELF file.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
#[doc(alias = "Elf64_Ehdr")]
pub struct Header64 {
    /// ELF identification magic number [0x7f, 'E', 'L', 'F']
    #[doc(alias = "EI_MAG0")]
    #[doc(alias = "EI_MAG1")]
    #[doc(alias = "EI_MAG2")]
    #[doc(alias = "EI_MAG3")]
    pub magic: Magic,
    /// Object file class (32-bit or 64-bit)
    #[doc(alias = "EI_CLASS")]
    pub class: Class,
    /// Data encoding (little-endian or big-endian)
    #[doc(alias = "EI_DATA")]
    pub encoding: Encoding,
    /// File version (EI_VERSION)
    #[doc(alias = "EI_VERSION")]
    pub header_version: HeaderVersion,
    /// Operating system/ABI identification
    #[doc(alias = "EI_OSABI")]
    pub os_abi: OsAbi,
    /// ABI version
    #[doc(alias = "EI_ABIVERSION")]
    pub abi_version: u8,
    /// Padding bytes (reserved for future use)
    #[doc(alias = "EI_PAD")]
    pub pad: [u8; 7],
    /// Object file type (executable, relocatable, shared object, etc.)
    #[doc(alias = "e_type")]
    pub object_type: ObjectType,
    /// Target architecture
    #[doc(alias = "e_machine")]
    pub machine: Machine,
    /// Object file version
    #[doc(alias = "e_version")]
    pub version: Version,
    /// Entry point virtual address
    #[doc(alias = "e_entry")]
    pub entry_point: u64,
    /// Program header table file offset
    #[doc(alias = "e_phoff")]
    pub program_header_offset: u64,
    /// Section header table file offset
    #[doc(alias = "e_shoff")]
    pub section_header_offset: u64,
    /// Processor-specific flags
    #[doc(alias = "e_flags")]
    pub flags: u32,
    /// ELF header size in bytes
    #[doc(alias = "e_ehsize")]
    pub header_size: u16,
    /// Program header table entry size
    #[doc(alias = "e_phentsize")]
    pub program_header_entry_size: u16,
    /// Number of entries in the program header table
    #[doc(alias = "e_phnum")]
    pub program_header_count: u16,
    /// Section header table entry size
    #[doc(alias = "e_shentsize")]
    pub section_header_entry_size: u16,
    /// Number of entries in the section header table
    #[doc(alias = "e_shnum")]
    pub section_header_count: u16,
    /// Section header string table index
    #[doc(alias = "e_shstrndx")]
    pub section_header_string_table_index: u16,
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

elf_enum! {
    pub struct Class(u8) {
        NONE = 0, "Invalid",
        ELF32 = 1, "ELF32",
        ELF64 = 2, "ELF64",
    }
}

elf_enum! {
    pub struct Encoding(u8) {
        NONE = 0, "Invalid",
        ELFDATA2LSB = 1, "2's complement, little endian",
        ELFDATA2MSB = 2, "2's complement, big endian",
    }
}

elf_enum! {
    pub struct HeaderVersion(u8) {
        /// Invalid version
        NONE = 0, "0 (invalid)",
        /// Current version
        CURRENT = 1, "1 (current)",
    }
}

elf_enum! {
    pub struct OsAbi(u8) {
        /// No extensions or unspecified
        NONE = 0, "UNIX - System V",
        /// Hewlett-Packard HP-UX
        HPUX = 1, "HP-UX",
        /// NetBSD
        NETBSD = 2, "NetBSD",
        /// GNU
        GNU = 3, "Linux",
        /// Linux (historical - alias for ELFOSABI_GNU)
        LINUX = 3, "Linux",
        /// Sun Solaris
        SOLARIS = 6, "Solaris",
        /// AIX
        AIX = 7, "AIX",
        /// IRIX
        IRIX = 8, "IRIX",
        /// FreeBSD
        FREEBSD = 9, "FreeBSD",
        /// Compaq TRU64 UNIX
        TRU64 = 10, "TRU64",
        /// Novell Modesto
        MODESTO = 11, "Modesto",
        /// Open BSD
        OPENBSD = 12, "OpenBSD",
        /// Open VMS
        OPENVMS = 13, "OpenVMS",
        /// Hewlett-Packard Non-Stop Kernel
        NSK = 14, "NSK",
        /// Amiga Research OS
        AROS = 15, "AROS",
        /// The FenixOS highly scalable multi-core OS
        FENIXOS = 16, "FenixOS",
        /// Nuxi CloudABI
        CLOUDABI = 17, "CloudABI",
        /// Stratus Technologies OpenVOS
        OPENVOS = 18, "OpenVOS",
    }
}

elf_enum! {
    pub struct ObjectType(u16) {
        NONE = 0, "NONE (No file type)",
        REL = 1, "REL (Relocatable file)",
        EXEC = 2, "EXEC (Executable file)",
        DYN = 3, "DYN (Shared object file)",
        CORE = 4, "CORE (Core file)",
        LOOS = 0xfe00, "OS-specific range start",
        HIOS = 0xfeff, "OS-specific range end",
        LOPROC = 0xff00, "Processor-specific range start",
        HIPROC = 0xffff, "Processor-specific range end",
    }
}

elf_enum! {
    pub struct Machine(u16) {
        /// No machine
        NONE = 0, "No machine",
        /// AT&T WE 32100
        M32 = 1, "AT&T WE 32100",
        /// SPARC
        SPARC = 2, "SPARC",
        /// Intel 80386
        _386 = 3, "Intel 80386",
        /// Motorola 68000
        _68K = 4, "Motorola 68000",
        /// Motorola 88000
        _88K = 5, "Motorola 88000",
        /// Intel MCU
        IAMCU = 6, "Intel MCU",
        /// Intel 80860
        _860 = 7, "Intel 80860",
        /// MIPS I Architecture
        MIPS = 8, "MIPS I Architecture",
        /// IBM System/370 Processor
        S370 = 9, "IBM System/370 Processor",
        /// MIPS RS3000 Little-endian
        MIPS_RS3_LE = 10, "MIPS RS3000 Little-endian",
        /// Hewlett-Packard PA-RISC
        PARISC = 15, "Hewlett-Packard PA-RISC",
        /// Fujitsu VPP500
        VPP500 = 17, "Fujitsu VPP500",
        /// Enhanced instruction set SPARC
        SPARC32PLUS = 18, "Enhanced instruction set SPARC",
        /// Intel 80960
        _960 = 19, "Intel 80960",
        /// PowerPC
        PPC = 20, "PowerPC",
        /// 64-bit PowerPC
        PPC64 = 21, "64-bit PowerPC",
        /// IBM System/390 Processor
        S390 = 22, "IBM System/390 Processor",
        /// IBM SPU/SPC
        SPU = 23, "IBM SPU/SPC",
        /// NEC V800
        V800 = 36, "NEC V800",
        /// Fujitsu FR20
        FR20 = 37, "Fujitsu FR20",
        /// TRW RH-32
        RH32 = 38, "TRW RH-32",
        /// Motorola RCE
        RCE = 39, "Motorola RCE",
        /// ARM 32-bit architecture (AARCH32)
        ARM = 40, "ARM",
        /// Digital Alpha
        ALPHA = 41, "Digital Alpha",
        /// Hitachi SH
        SH = 42, "Hitachi SH",
        /// SPARC Version 9
        SPARCV9 = 43, "SPARC Version 9",
        /// Siemens TriCore embedded processor
        TRICORE = 44, "Siemens TriCore embedded processor",
        /// Argonaut RISC Core, Argonaut Technologies Inc.
        ARC = 45, "Argonaut RISC Core",
        /// Hitachi H8/300
        H8_300 = 46, "Hitachi H8/300",
        /// Hitachi H8/300H
        H8_300H = 47, "Hitachi H8/300H",
        /// Hitachi H8S
        H8S = 48, "Hitachi H8S",
        /// Hitachi H8/500
        H8_500 = 49, "Hitachi H8/500",
        /// Intel IA-64 processor architecture
        IA_64 = 50, "Intel IA-64 processor architecture",
        /// Stanford MIPS-X
        MIPS_X = 51, "Stanford MIPS-X",
        /// Motorola ColdFire
        COLDFIRE = 52, "Motorola ColdFire",
        /// Motorola M68HC12
        _68HC12 = 53, "Motorola M68HC12",
        /// Fujitsu MMA Multimedia Accelerator
        MMA = 54, "Fujitsu MMA Multimedia Accelerator",
        /// Siemens PCP
        PCP = 55, "Siemens PCP",
        /// Sony nCPU embedded RISC processor
        NCPU = 56, "Sony nCPU embedded RISC processor",
        /// Denso NDR1 microprocessor
        NDR1 = 57, "Denso NDR1 microprocessor",
        /// Motorola Star*Core processor
        STARCORE = 58, "Motorola Star*Core processor",
        /// Toyota ME16 processor
        ME16 = 59, "Toyota ME16 processor",
        /// STMicroelectronics ST100 processor
        ST100 = 60, "STMicroelectronics ST100 processor",
        /// Advanced Logic Corp. TinyJ embedded processor family
        TINYJ = 61, "Advanced Logic Corp. TinyJ embedded processor family",
        /// AMD x86-64 architecture
        X86_64 = 62, "Advanced Micro Devices X86-64",
        /// Sony DSP Processor
        PDSP = 63, "Sony DSP Processor",
        /// Digital Equipment Corp. PDP-10
        PDP10 = 64, "Digital Equipment Corp. PDP-10",
        /// Digital Equipment Corp. PDP-11
        PDP11 = 65, "Digital Equipment Corp. PDP-11",
        /// Siemens FX66 microcontroller
        FX66 = 66, "Siemens FX66 microcontroller",
        /// STMicroelectronics ST9+ 8/16 bit microcontroller
        ST9PLUS = 67, "STMicroelectronics ST9+ 8/16 bit microcontroller",
        /// STMicroelectronics ST7 8-bit microcontroller
        ST7 = 68, "STMicroelectronics ST7 8-bit microcontroller",
        /// Motorola MC68HC16 Microcontroller
        _68HC16 = 69, "Motorola MC68HC16 Microcontroller",
        /// Motorola MC68HC11 Microcontroller
        _68HC11 = 70, "Motorola MC68HC11 Microcontroller",
        /// Motorola MC68HC08 Microcontroller
        _68HC08 = 71, "Motorola MC68HC08 Microcontroller",
        /// Motorola MC68HC05 Microcontroller
        _68HC05 = 72, "Motorola MC68HC05 Microcontroller",
        /// Silicon Graphics SVx
        SVX = 73, "Silicon Graphics SVx",
        /// STMicroelectronics ST19 8-bit microcontroller
        ST19 = 74, "STMicroelectronics ST19 8-bit microcontroller",
        /// Digital VAX
        VAX = 75, "Digital VAX",
        /// Axis Communications 32-bit embedded processor
        CRIS = 76, "Axis Communications 32-bit embedded processor",
        /// Infineon Technologies 32-bit embedded processor
        JAVELIN = 77, "Infineon Technologies 32-bit embedded processor",
        /// Element 14 64-bit DSP Processor
        FIREPATH = 78, "Element 14 64-bit DSP Processor",
        /// LSI Logic 16-bit DSP Processor
        ZSP = 79, "LSI Logic 16-bit DSP Processor",
        /// Donald Knuth's educational 64-bit processor
        MMIX = 80, "Donald Knuth's educational 64-bit processor",
        /// Harvard University machine-independent object files
        HUANY = 81, "Harvard University machine-independent object files",
        /// SiTera Prism
        PRISM = 82, "SiTera Prism",
        /// Atmel AVR 8-bit microcontroller
        AVR = 83, "Atmel AVR 8-bit microcontroller",
        /// Fujitsu FR30
        FR30 = 84, "Fujitsu FR30",
        /// Mitsubishi D10V
        D10V = 85, "Mitsubishi D10V",
        /// Mitsubishi D30V
        D30V = 86, "Mitsubishi D30V",
        /// NEC v850
        V850 = 87, "NEC v850",
        /// Mitsubishi M32R
        M32R = 88, "Mitsubishi M32R",
        /// Matsushita MN10300
        MN10300 = 89, "Matsushita MN10300",
        /// Matsushita MN10200
        MN10200 = 90, "Matsushita MN10200",
        /// picoJava
        PJ = 91, "picoJava",
        /// OpenRISC 32-bit embedded processor
        OPENRISC = 92, "OpenRISC 32-bit embedded processor",
        /// ARC International ARCompact processor
        ARC_COMPACT = 93, "ARC International ARCompact processor",
        /// Tensilica Xtensa Architecture
        XTENSA = 94, "Tensilica Xtensa Architecture",
        /// Alphamosaic VideoCore processor
        VIDEOCORE = 95, "Alphamosaic VideoCore processor",
        /// Thompson Multimedia General Purpose Processor
        TMM_GPP = 96, "Thompson Multimedia General Purpose Processor",
        /// National Semiconductor 32000 series
        NS32K = 97, "National Semiconductor 32000 series",
        /// Tenor Network TPC processor
        TPC = 98, "Tenor Network TPC processor",
        /// Trebia SNP 1000 processor
        SNP1K = 99, "Trebia SNP 1000 processor",
        /// STMicroelectronics ST200 microcontroller
        ST200 = 100, "STMicroelectronics ST200 microcontroller",
        /// Ubicom IP2xxx microcontroller family
        IP2K = 101, "Ubicom IP2xxx microcontroller family",
        /// MAX Processor
        MAX = 102, "MAX Processor",
        /// National Semiconductor CompactRISC microprocessor
        CR = 103, "National Semiconductor CompactRISC microprocessor",
        /// Fujitsu F2MC16
        F2MC16 = 104, "Fujitsu F2MC16",
        /// Texas Instruments embedded microcontroller msp430
        MSP430 = 105, "Texas Instruments embedded microcontroller msp430",
        /// Analog Devices Blackfin (DSP) processor
        BLACKFIN = 106, "Analog Devices Blackfin (DSP) processor",
        /// S1C33 Family of Seiko Epson processors
        SE_C33 = 107, "S1C33 Family of Seiko Epson processors",
        /// Sharp embedded microprocessor
        SEP = 108, "Sharp embedded microprocessor",
        /// Arca RISC Microprocessor
        ARCA = 109, "Arca RISC Microprocessor",
        /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
        UNICORE = 110, "PKU-Unity UNICORE",
        /// eXcess: 16/32/64-bit configurable embedded CPU
        EXCESS = 111, "eXcess: 16/32/64-bit configurable embedded CPU",
        /// Icera Semiconductor Inc. Deep Execution Processor
        DXP = 112, "Icera Semiconductor Inc. Deep Execution Processor",
        /// Altera Nios II soft-core processor
        ALTERA_NIOS2 = 113, "Altera Nios II soft-core processor",
        /// National Semiconductor CompactRISC CRX microprocessor
        CRX = 114, "National Semiconductor CompactRISC CRX microprocessor",
        /// Motorola XGATE embedded processor
        XGATE = 115, "Motorola XGATE embedded processor",
        /// Infineon C16x/XC16x processor
        C166 = 116, "Infineon C16x/XC16x processor",
        /// Renesas M16C series microprocessors
        M16C = 117, "Renesas M16C series microprocessors",
        /// Microchip Technology dsPIC30F Digital Signal Controller
        DSPIC30F = 118, "Microchip dsPIC30F",
        /// Freescale Communication Engine RISC core
        CE = 119, "Freescale Communication Engine RISC core",
        /// Renesas M32C series microprocessors
        M32C = 120, "Renesas M32C series microprocessors",
        /// Altium TSK3000 core
        TSK3000 = 131, "Altium TSK3000 core",
        /// Freescale RS08 embedded processor
        RS08 = 132, "Freescale RS08 embedded processor",
        /// Analog Devices SHARC family of 32-bit DSP processors
        SHARC = 133, "Analog Devices SHARC",
        /// Cyan Technology eCOG2 microprocessor
        ECOG2 = 134, "Cyan Technology eCOG2 microprocessor",
        /// Sunplus S+core7 RISC processor
        SCORE7 = 135, "Sunplus S+core7 RISC processor",
        /// New Japan Radio (NJR) 24-bit DSP Processor
        DSP24 = 136, "New Japan Radio (NJR) 24-bit DSP Processor",
        /// Broadcom VideoCore III processor
        VIDEOCORE3 = 137, "Broadcom VideoCore III processor",
        /// RISC processor for Lattice FPGA architecture
        LATTICEMICO32 = 138, "Lattice MICO32",
        /// Seiko Epson C17 family
        SE_C17 = 139, "Seiko Epson C17 family",
        /// The Texas Instruments TMS320C6000 DSP family
        TI_C6000 = 140, "The Texas Instruments TMS320C6000 DSP family",
        /// The Texas Instruments TMS320C2000 DSP family
        TI_C2000 = 141, "The Texas Instruments TMS320C2000 DSP family",
        /// The Texas Instruments TMS320C55x DSP family
        TI_C5500 = 142, "The Texas Instruments TMS320C55x DSP family",
        /// Texas Instruments Application Specific RISC Processor, 32bit fetch
        TI_ARP32 = 143, "Texas Instruments ARP32",
        /// Texas Instruments Programmable Realtime Unit
        TI_PRU = 144, "Texas Instruments Programmable Realtime Unit",
        /// STMicroelectronics 64bit VLIW Data Signal Processor
        MMDSP_PLUS = 160, "STMicroelectronics MMDSP+",
        /// Cypress M8C microprocessor
        CYPRESS_M8C = 161, "Cypress M8C microprocessor",
        /// Renesas R32C series microprocessors
        R32C = 162, "Renesas R32C series microprocessors",
        /// NXP Semiconductors TriMedia architecture family
        TRIMEDIA = 163, "NXP Semiconductors TriMedia architecture family",
        /// QUALCOMM DSP6 Processor
        QDSP6 = 164, "QUALCOMM DSP6 Processor",
        /// Intel 8051 and variants
        _8051 = 165, "Intel 8051 and variants",
        /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
        STXP7X = 166, "STMicroelectronics STxP7x",
        /// Andes Technology compact code size embedded RISC processor family
        NDS32 = 167, "Andes Technology NDS32",
        /// Cyan Technology eCOG1X family
        ECOG1X = 168, "Cyan Technology eCOG1X family",
        /// Dallas Semiconductor MAXQ30 Core Micro-controllers
        MAXQ30 = 169, "Dallas Semiconductor MAXQ30",
        /// New Japan Radio (NJR) 16-bit DSP Processor
        XIMO16 = 170, "New Japan Radio XIMO16",
        /// M2000 Reconfigurable RISC Microprocessor
        MANIK = 171, "M2000 MANIK",
        /// Cray Inc. NV2 vector architecture
        CRAYNV2 = 172, "Cray NV2",
        /// Renesas RX family
        RX = 173, "Renesas RX",
        /// Imagination Technologies META processor architecture
        METAG = 174, "Imagination Technologies META",
        /// MCST Elbrus general purpose hardware architecture
        MCST_ELBRUS = 175, "MCST Elbrus",
        /// Cyan Technology eCOG16 family
        ECOG16 = 176, "Cyan Technology eCOG16 family",
        /// National Semiconductor CompactRISC CR16 16-bit microprocessor
        CR16 = 177, "National Semiconductor CR16",
        /// Freescale Extended Time Processing Unit
        ETPU = 178, "Freescale ETPU",
        /// Infineon Technologies SLE9X core
        SLE9X = 179, "Infineon SLE9X",
        /// Intel L10M
        L10M = 180, "Intel L10M",
        /// Intel K10M
        K10M = 181, "Intel K10M",
        /// ARM 64-bit architecture (AARCH64)
        AARCH64 = 183, "AArch64",
        /// Atmel Corporation 32-bit microprocessor family
        AVR32 = 185, "Atmel Corporation 32-bit microprocessor family",
        /// STMicroeletronics STM8 8-bit microcontroller
        STM8 = 186, "STMicroeletronics STM8 8-bit microcontroller",
        /// Tilera TILE64 multicore architecture family
        TILE64 = 187, "Tilera TILE64 multicore architecture family",
        /// Tilera TILEPro multicore architecture family
        TILEPRO = 188, "Tilera TILEPro multicore architecture family",
        /// Xilinx MicroBlaze 32-bit RISC soft processor core
        MICROBLAZE = 189, "Xilinx MicroBlaze",
        /// NVIDIA CUDA architecture
        CUDA = 190, "NVIDIA CUDA architecture",
        /// Tilera TILE-Gx multicore architecture family
        TILEGX = 191, "Tilera TILE-Gx multicore architecture family",
        /// CloudShield architecture family
        CLOUDSHIELD = 192, "CloudShield architecture family",
        /// KIPO-KAIST Core-A 1st generation processor family
        COREA_1ST = 193, "KIPO-KAIST Core-A 1st gen",
        /// KIPO-KAIST Core-A 2nd generation processor family
        COREA_2ND = 194, "KIPO-KAIST Core-A 2nd gen",
        /// Synopsys ARCompact V2
        ARC_COMPACT2 = 195, "Synopsys ARCompact V2",
        /// Open8 8-bit RISC soft processor core
        OPEN8 = 196, "Open8 8-bit RISC soft processor core",
        /// Renesas RL78 family
        RL78 = 197, "Renesas RL78 family",
        /// Broadcom VideoCore V processor
        VIDEOCORE5 = 198, "Broadcom VideoCore V processor",
        /// Renesas 78KOR family
        _78KOR = 199, "Renesas 78KOR family",
        /// Freescale 56800EX Digital Signal Controller (DSC)
        _56800EX = 200, "Freescale 56800EX",
        /// Beyond BA1 CPU architecture
        BA1 = 201, "Beyond BA1 CPU architecture",
        /// Beyond BA2 CPU architecture
        BA2 = 202, "Beyond BA2 CPU architecture",
        /// XMOS xCORE processor family
        XCORE = 203, "XMOS xCORE processor family",
        /// Microchip 8-bit PIC(r) family
        MCHP_PIC = 204, "Microchip 8-bit PIC(r) family",
        /// KM211 KM32 32-bit processor
        KM32 = 210, "KM211 KM32 32-bit processor",
        /// KM211 KMX32 32-bit processor
        KMX32 = 211, "KM211 KMX32 32-bit processor",
        /// KM211 KMX16 16-bit processor
        KMX16 = 212, "KM211 KMX16 16-bit processor",
        /// KM211 KMX8 8-bit processor
        KMX8 = 213, "KM211 KMX8 8-bit processor",
        /// KM211 KVARC processor
        KVARC = 214, "KM211 KVARC processor",
        /// Paneve CDP architecture family
        CDP = 215, "Paneve CDP architecture family",
        /// Cognitive Smart Memory Processor
        COGE = 216, "Cognitive Smart Memory Processor",
        /// Bluechip Systems CoolEngine
        COOL = 217, "Bluechip Systems CoolEngine",
        /// Nanoradio Optimized RISC
        NORC = 218, "Nanoradio Optimized RISC",
        /// CSR Kalimba architecture family
        CSR_KALIMBA = 219, "CSR Kalimba architecture family",
        /// Zilog Z80
        Z80 = 220, "Zilog Z80",
        /// Controls and Data Services VISIUMcore processor
        VISIUM = 221, "Controls and Data Services VISIUMcore processor",
        /// FTDI Chip FT32 high performance 32-bit RISC architecture
        FT32 = 222, "FTDI FT32",
        /// Moxie processor family
        MOXIE = 223, "Moxie processor family",
        /// AMD GPU architecture
        AMDGPU = 224, "AMD GPU architecture",
        /// RISC-V
        RISCV = 243, "RISC-V",
    }
}

elf_enum! {
    pub struct Version(u32) {
        /// Invalid version
        NONE = 0, "0x0 (invalid)",
        /// Current version
        CURRENT = 1, "0x1",
    }
}
