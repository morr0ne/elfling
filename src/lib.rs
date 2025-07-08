use zerocopy::FromBytes;

macro_rules! elf_enum {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($inner_type:ty) {
            $(
                $(#[doc = $doc:expr])*
                $const_name:ident = $value:expr,
            )*
        }
    ) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
        $(#[$attr])*
        $vis struct $name {
            inner: $inner_type,
        }

        impl $name {
            $(
                $(#[doc = $doc])*
                pub const $const_name: Self = Self::from_raw($value);
            )*

            pub const fn from_raw(raw: $inner_type) -> Self {
                Self { inner: raw }
            }

            pub const fn into_raw(self) -> $inner_type {
                self.inner
            }

            pub const fn as_raw(&self) -> $inner_type {
                self.inner
            }
        }

        impl From<$name> for $inner_type {
            fn from(e: $name) -> $inner_type {
                e.into_raw()
            }
        }
    };
}

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
    pub machine: Machine,
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

elf_enum! {
    pub struct Class(u8) {
        NONE = 0,
        ELF32 = 1,
        ELF64 = 2,
    }
}

elf_enum! {
    pub struct Encoding(u8) {
        NONE = 0,
        ELFDATA2LSB = 1,
        ELFDATA2MSB = 2,
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct OsAbi {
    inner: u8,
}

elf_enum! {
    pub struct ObjectType(u16) {
        NONE = 0,
        REL = 1,
        EXEC = 2,
        DYN = 3,
        CORE = 4,
        LOOS = 0xfe00,
        HIOS = 0xfeff,
        LOPROC = 0xff00,
        HIPROC = 0xffff,
    }
}

elf_enum! {
    pub struct Machine(u16) {
        /// No machine
        NONE = 0,
        /// AT&T WE 32100
        M32 = 1,
        /// SPARC
        SPARC = 2,
        /// Intel 80386
        _386 = 3,
        /// Motorola 68000
        _68K = 4,
        /// Motorola 88000
        _88K = 5,
        /// Intel MCU
        IAMCU = 6,
        /// Intel 80860
        _860 = 7,
        /// MIPS I Architecture
        MIPS = 8,
        /// IBM System/370 Processor
        S370 = 9,
        /// MIPS RS3000 Little-endian
        MIPS_RS3_LE = 10,
        /// Hewlett-Packard PA-RISC
        PARISC = 15,
        /// Fujitsu VPP500
        VPP500 = 17,
        /// Enhanced instruction set SPARC
        SPARC32PLUS = 18,
        /// Intel 80960
        _960 = 19,
        /// PowerPC
        PPC = 20,
        /// 64-bit PowerPC
        PPC64 = 21,
        /// IBM System/390 Processor
        S390 = 22,
        /// IBM SPU/SPC
        SPU = 23,
        /// NEC V800
        V800 = 36,
        /// Fujitsu FR20
        FR20 = 37,
        /// TRW RH-32
        RH32 = 38,
        /// Motorola RCE
        RCE = 39,
        /// ARM 32-bit architecture (AARCH32)
        ARM = 40,
        /// Digital Alpha
        ALPHA = 41,
        /// Hitachi SH
        SH = 42,
        /// SPARC Version 9
        SPARCV9 = 43,
        /// Siemens TriCore embedded processor
        TRICORE = 44,
        /// Argonaut RISC Core, Argonaut Technologies Inc.
        ARC = 45,
        /// Hitachi H8/300
        H8_300 = 46,
        /// Hitachi H8/300H
        H8_300H = 47,
        /// Hitachi H8S
        H8S = 48,
        /// Hitachi H8/500
        H8_500 = 49,
        /// Intel IA-64 processor architecture
        IA_64 = 50,
        /// Stanford MIPS-X
        MIPS_X = 51,
        /// Motorola ColdFire
        COLDFIRE = 52,
        /// Motorola M68HC12
        _68HC12 = 53,
        /// Fujitsu MMA Multimedia Accelerator
        MMA = 54,
        /// Siemens PCP
        PCP = 55,
        /// Sony nCPU embedded RISC processor
        NCPU = 56,
        /// Denso NDR1 microprocessor
        NDR1 = 57,
        /// Motorola Star*Core processor
        STARCORE = 58,
        /// Toyota ME16 processor
        ME16 = 59,
        /// STMicroelectronics ST100 processor
        ST100 = 60,
        /// Advanced Logic Corp. TinyJ embedded processor family
        TINYJ = 61,
        /// AMD x86-64 architecture
        X86_64 = 62,
        /// Sony DSP Processor
        PDSP = 63,
        /// Digital Equipment Corp. PDP-10
        PDP10 = 64,
        /// Digital Equipment Corp. PDP-11
        PDP11 = 65,
        /// Siemens FX66 microcontroller
        FX66 = 66,
        /// STMicroelectronics ST9+ 8/16 bit microcontroller
        ST9PLUS = 67,
        /// STMicroelectronics ST7 8-bit microcontroller
        ST7 = 68,
        /// Motorola MC68HC16 Microcontroller
        _68HC16 = 69,
        /// Motorola MC68HC11 Microcontroller
        _68HC11 = 70,
        /// Motorola MC68HC08 Microcontroller
        _68HC08 = 71,
        /// Motorola MC68HC05 Microcontroller
        _68HC05 = 72,
        /// Silicon Graphics SVx
        SVX = 73,
        /// STMicroelectronics ST19 8-bit microcontroller
        ST19 = 74,
        /// Digital VAX
        VAX = 75,
        /// Axis Communications 32-bit embedded processor
        CRIS = 76,
        /// Infineon Technologies 32-bit embedded processor
        JAVELIN = 77,
        /// Element 14 64-bit DSP Processor
        FIREPATH = 78,
        /// LSI Logic 16-bit DSP Processor
        ZSP = 79,
        /// Donald Knuth's educational 64-bit processor
        MMIX = 80,
        /// Harvard University machine-independent object files
        HUANY = 81,
        /// SiTera Prism
        PRISM = 82,
        /// Atmel AVR 8-bit microcontroller
        AVR = 83,
        /// Fujitsu FR30
        FR30 = 84,
        /// Mitsubishi D10V
        D10V = 85,
        /// Mitsubishi D30V
        D30V = 86,
        /// NEC v850
        V850 = 87,
        /// Mitsubishi M32R
        M32R = 88,
        /// Matsushita MN10300
        MN10300 = 89,
        /// Matsushita MN10200
        MN10200 = 90,
        /// picoJava
        PJ = 91,
        /// OpenRISC 32-bit embedded processor
        OPENRISC = 92,
        /// ARC International ARCompact processor
        ARC_COMPACT = 93,
        /// Tensilica Xtensa Architecture
        XTENSA = 94,
        /// Alphamosaic VideoCore processor
        VIDEOCORE = 95,
        /// Thompson Multimedia General Purpose Processor
        TMM_GPP = 96,
        /// National Semiconductor 32000 series
        NS32K = 97,
        /// Tenor Network TPC processor
        TPC = 98,
        /// Trebia SNP 1000 processor
        SNP1K = 99,
        /// STMicroelectronics ST200 microcontroller
        ST200 = 100,
        /// Ubicom IP2xxx microcontroller family
        IP2K = 101,
        /// MAX Processor
        MAX = 102,
        /// National Semiconductor CompactRISC microprocessor
        CR = 103,
        /// Fujitsu F2MC16
        F2MC16 = 104,
        /// Texas Instruments embedded microcontroller msp430
        MSP430 = 105,
        /// Analog Devices Blackfin (DSP) processor
        BLACKFIN = 106,
        /// S1C33 Family of Seiko Epson processors
        SE_C33 = 107,
        /// Sharp embedded microprocessor
        SEP = 108,
        /// Arca RISC Microprocessor
        ARCA = 109,
        /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
        UNICORE = 110,
        /// eXcess: 16/32/64-bit configurable embedded CPU
        EXCESS = 111,
        /// Icera Semiconductor Inc. Deep Execution Processor
        DXP = 112,
        /// Altera Nios II soft-core processor
        ALTERA_NIOS2 = 113,
        /// National Semiconductor CompactRISC CRX microprocessor
        CRX = 114,
        /// Motorola XGATE embedded processor
        XGATE = 115,
        /// Infineon C16x/XC16x processor
        C166 = 116,
        /// Renesas M16C series microprocessors
        M16C = 117,
        /// Microchip Technology dsPIC30F Digital Signal Controller
        DSPIC30F = 118,
        /// Freescale Communication Engine RISC core
        CE = 119,
        /// Renesas M32C series microprocessors
        M32C = 120,
        /// Altium TSK3000 core
        TSK3000 = 131,
        /// Freescale RS08 embedded processor
        RS08 = 132,
        /// Analog Devices SHARC family of 32-bit DSP processors
        SHARC = 133,
        /// Cyan Technology eCOG2 microprocessor
        ECOG2 = 134,
        /// Sunplus S+core7 RISC processor
        SCORE7 = 135,
        /// New Japan Radio (NJR) 24-bit DSP Processor
        DSP24 = 136,
        /// Broadcom VideoCore III processor
        VIDEOCORE3 = 137,
        /// RISC processor for Lattice FPGA architecture
        LATTICEMICO32 = 138,
        /// Seiko Epson C17 family
        SE_C17 = 139,
        /// The Texas Instruments TMS320C6000 DSP family
        TI_C6000 = 140,
        /// The Texas Instruments TMS320C2000 DSP family
        TI_C2000 = 141,
        /// The Texas Instruments TMS320C55x DSP family
        TI_C5500 = 142,
        /// Texas Instruments Application Specific RISC Processor, 32bit fetch
        TI_ARP32 = 143,
        /// Texas Instruments Programmable Realtime Unit
        TI_PRU = 144,
        /// STMicroelectronics 64bit VLIW Data Signal Processor
        MMDSP_PLUS = 160,
        /// Cypress M8C microprocessor
        CYPRESS_M8C = 161,
        /// Renesas R32C series microprocessors
        R32C = 162,
        /// NXP Semiconductors TriMedia architecture family
        TRIMEDIA = 163,
        /// QUALCOMM DSP6 Processor
        QDSP6 = 164,
        /// Intel 8051 and variants
        _8051 = 165,
        /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
        STXP7X = 166,
        /// Andes Technology compact code size embedded RISC processor family
        NDS32 = 167,
        /// Cyan Technology eCOG1X family
        ECOG1X = 168,
        /// Dallas Semiconductor MAXQ30 Core Micro-controllers
        MAXQ30 = 169,
        /// New Japan Radio (NJR) 16-bit DSP Processor
        XIMO16 = 170,
        /// M2000 Reconfigurable RISC Microprocessor
        MANIK = 171,
        /// Cray Inc. NV2 vector architecture
        CRAYNV2 = 172,
        /// Renesas RX family
        RX = 173,
        /// Imagination Technologies META processor architecture
        METAG = 174,
        /// MCST Elbrus general purpose hardware architecture
        MCST_ELBRUS = 175,
        /// Cyan Technology eCOG16 family
        ECOG16 = 176,
        /// National Semiconductor CompactRISC CR16 16-bit microprocessor
        CR16 = 177,
        /// Freescale Extended Time Processing Unit
        ETPU = 178,
        /// Infineon Technologies SLE9X core
        SLE9X = 179,
        /// Intel L10M
        L10M = 180,
        /// Intel K10M
        K10M = 181,
        /// ARM 64-bit architecture (AARCH64)
        AARCH64 = 183,
        /// Atmel Corporation 32-bit microprocessor family
        AVR32 = 185,
        /// STMicroeletronics STM8 8-bit microcontroller
        STM8 = 186,
        /// Tilera TILE64 multicore architecture family
        TILE64 = 187,
        /// Tilera TILEPro multicore architecture family
        TILEPRO = 188,
        /// Xilinx MicroBlaze 32-bit RISC soft processor core
        MICROBLAZE = 189,
        /// NVIDIA CUDA architecture
        CUDA = 190,
        /// Tilera TILE-Gx multicore architecture family
        TILEGX = 191,
        /// CloudShield architecture family
        CLOUDSHIELD = 192,
        /// KIPO-KAIST Core-A 1st generation processor family
        COREA_1ST = 193,
        /// KIPO-KAIST Core-A 2nd generation processor family
        COREA_2ND = 194,
        /// Synopsys ARCompact V2
        ARC_COMPACT2 = 195,
        /// Open8 8-bit RISC soft processor core
        OPEN8 = 196,
        /// Renesas RL78 family
        RL78 = 197,
        /// Broadcom VideoCore V processor
        VIDEOCORE5 = 198,
        /// Renesas 78KOR family
        _78KOR = 199,
        /// Freescale 56800EX Digital Signal Controller (DSC)
        _56800EX = 200,
        /// Beyond BA1 CPU architecture
        BA1 = 201,
        /// Beyond BA2 CPU architecture
        BA2 = 202,
        /// XMOS xCORE processor family
        XCORE = 203,
        /// Microchip 8-bit PIC(r) family
        MCHP_PIC = 204,
        /// KM211 KM32 32-bit processor
        KM32 = 210,
        /// KM211 KMX32 32-bit processor
        KMX32 = 211,
        /// KM211 KMX16 16-bit processor
        KMX16 = 212,
        /// KM211 KMX8 8-bit processor
        KMX8 = 213,
        /// KM211 KVARC processor
        KVARC = 214,
        /// Paneve CDP architecture family
        CDP = 215,
        /// Cognitive Smart Memory Processor
        COGE = 216,
        /// Bluechip Systems CoolEngine
        COOL = 217,
        /// Nanoradio Optimized RISC
        NORC = 218,
        /// CSR Kalimba architecture family
        CSR_KALIMBA = 219,
        /// Zilog Z80
        Z80 = 220,
        /// Controls and Data Services VISIUMcore processor
        VISIUM = 221,
        /// FTDI Chip FT32 high performance 32-bit RISC architecture
        FT32 = 222,
        /// Moxie processor family
        MOXIE = 223,
        /// AMD GPU architecture
        AMDGPU = 224,
        /// RISC-V
        RISCV = 243,
    }
}
