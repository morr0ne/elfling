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

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, FromBytes)]
pub struct Machine {
    inner: u16,
}

impl Machine {
    /// No machine
    pub const NONE: Self = Self::from_raw(0);
    /// AT&T WE 32100
    pub const M32: Self = Self::from_raw(1);
    /// SPARC
    pub const SPARC: Self = Self::from_raw(2);
    /// Intel 80386
    pub const _386: Self = Self::from_raw(3);
    /// Motorola 68000
    pub const _68K: Self = Self::from_raw(4);
    /// Motorola 88000
    pub const _88K: Self = Self::from_raw(5);
    /// Intel MCU
    pub const IAMCU: Self = Self::from_raw(6);
    /// Intel 80860
    pub const _860: Self = Self::from_raw(7);
    /// MIPS I Architecture
    pub const MIPS: Self = Self::from_raw(8);
    /// IBM System/370 Processor
    pub const S370: Self = Self::from_raw(9);
    /// MIPS RS3000 Little-endian
    pub const MIPS_RS3_LE: Self = Self::from_raw(10);
    /// Hewlett-Packard PA-RISC
    pub const PARISC: Self = Self::from_raw(15);
    /// Fujitsu VPP500
    pub const VPP500: Self = Self::from_raw(17);
    /// Enhanced instruction set SPARC
    pub const SPARC32PLUS: Self = Self::from_raw(18);
    /// Intel 80960
    pub const _960: Self = Self::from_raw(19);
    /// PowerPC
    pub const PPC: Self = Self::from_raw(20);
    /// 64-bit PowerPC
    pub const PPC64: Self = Self::from_raw(21);
    /// IBM System/390 Processor
    pub const S390: Self = Self::from_raw(22);
    /// IBM SPU/SPC
    pub const SPU: Self = Self::from_raw(23);
    /// NEC V800
    pub const V800: Self = Self::from_raw(36);
    /// Fujitsu FR20
    pub const FR20: Self = Self::from_raw(37);
    /// TRW RH-32
    pub const RH32: Self = Self::from_raw(38);
    /// Motorola RCE
    pub const RCE: Self = Self::from_raw(39);
    /// ARM 32-bit architecture (AARCH32)
    pub const ARM: Self = Self::from_raw(40);
    /// Digital Alpha
    pub const ALPHA: Self = Self::from_raw(41);
    /// Hitachi SH
    pub const SH: Self = Self::from_raw(42);
    /// SPARC Version 9
    pub const SPARCV9: Self = Self::from_raw(43);
    /// Siemens TriCore embedded processor
    pub const TRICORE: Self = Self::from_raw(44);
    /// Argonaut RISC Core, Argonaut Technologies Inc.
    pub const ARC: Self = Self::from_raw(45);
    /// Hitachi H8/300
    pub const H8_300: Self = Self::from_raw(46);
    /// Hitachi H8/300H
    pub const H8_300H: Self = Self::from_raw(47);
    /// Hitachi H8S
    pub const H8S: Self = Self::from_raw(48);
    /// Hitachi H8/500
    pub const H8_500: Self = Self::from_raw(49);
    /// Intel IA-64 processor architecture
    pub const IA_64: Self = Self::from_raw(50);
    /// Stanford MIPS-X
    pub const MIPS_X: Self = Self::from_raw(51);
    /// Motorola ColdFire
    pub const COLDFIRE: Self = Self::from_raw(52);
    /// Motorola M68HC12
    pub const _68HC12: Self = Self::from_raw(53);
    /// Fujitsu MMA Multimedia Accelerator
    pub const MMA: Self = Self::from_raw(54);
    /// Siemens PCP
    pub const PCP: Self = Self::from_raw(55);
    /// Sony nCPU embedded RISC processor
    pub const NCPU: Self = Self::from_raw(56);
    /// Denso NDR1 microprocessor
    pub const NDR1: Self = Self::from_raw(57);
    /// Motorola Star*Core processor
    pub const STARCORE: Self = Self::from_raw(58);
    /// Toyota ME16 processor
    pub const ME16: Self = Self::from_raw(59);
    /// STMicroelectronics ST100 processor
    pub const ST100: Self = Self::from_raw(60);
    /// Advanced Logic Corp. TinyJ embedded processor family
    pub const TINYJ: Self = Self::from_raw(61);
    /// AMD x86-64 architecture
    pub const X86_64: Self = Self::from_raw(62);
    /// Sony DSP Processor
    pub const PDSP: Self = Self::from_raw(63);
    /// Digital Equipment Corp. PDP-10
    pub const PDP10: Self = Self::from_raw(64);
    /// Digital Equipment Corp. PDP-11
    pub const PDP11: Self = Self::from_raw(65);
    /// Siemens FX66 microcontroller
    pub const FX66: Self = Self::from_raw(66);
    /// STMicroelectronics ST9+ 8/16 bit microcontroller
    pub const ST9PLUS: Self = Self::from_raw(67);
    /// STMicroelectronics ST7 8-bit microcontroller
    pub const ST7: Self = Self::from_raw(68);
    /// Motorola MC68HC16 Microcontroller
    pub const _68HC16: Self = Self::from_raw(69);
    /// Motorola MC68HC11 Microcontroller
    pub const _68HC11: Self = Self::from_raw(70);
    /// Motorola MC68HC08 Microcontroller
    pub const _68HC08: Self = Self::from_raw(71);
    /// Motorola MC68HC05 Microcontroller
    pub const _68HC05: Self = Self::from_raw(72);
    /// Silicon Graphics SVx
    pub const SVX: Self = Self::from_raw(73);
    /// STMicroelectronics ST19 8-bit microcontroller
    pub const ST19: Self = Self::from_raw(74);
    /// Digital VAX
    pub const VAX: Self = Self::from_raw(75);
    /// Axis Communications 32-bit embedded processor
    pub const CRIS: Self = Self::from_raw(76);
    /// Infineon Technologies 32-bit embedded processor
    pub const JAVELIN: Self = Self::from_raw(77);
    /// Element 14 64-bit DSP Processor
    pub const FIREPATH: Self = Self::from_raw(78);
    /// LSI Logic 16-bit DSP Processor
    pub const ZSP: Self = Self::from_raw(79);
    /// Donald Knuth's educational 64-bit processor
    pub const MMIX: Self = Self::from_raw(80);
    /// Harvard University machine-independent object files
    pub const HUANY: Self = Self::from_raw(81);
    /// SiTera Prism
    pub const PRISM: Self = Self::from_raw(82);
    /// Atmel AVR 8-bit microcontroller
    pub const AVR: Self = Self::from_raw(83);
    /// Fujitsu FR30
    pub const FR30: Self = Self::from_raw(84);
    /// Mitsubishi D10V
    pub const D10V: Self = Self::from_raw(85);
    /// Mitsubishi D30V
    pub const D30V: Self = Self::from_raw(86);
    /// NEC v850
    pub const V850: Self = Self::from_raw(87);
    /// Mitsubishi M32R
    pub const M32R: Self = Self::from_raw(88);
    /// Matsushita MN10300
    pub const MN10300: Self = Self::from_raw(89);
    /// Matsushita MN10200
    pub const MN10200: Self = Self::from_raw(90);
    /// picoJava
    pub const PJ: Self = Self::from_raw(91);
    /// OpenRISC 32-bit embedded processor
    pub const OPENRISC: Self = Self::from_raw(92);
    /// ARC International ARCompact processor
    pub const ARC_COMPACT: Self = Self::from_raw(93);
    /// Tensilica Xtensa Architecture
    pub const XTENSA: Self = Self::from_raw(94);
    /// Alphamosaic VideoCore processor
    pub const VIDEOCORE: Self = Self::from_raw(95);
    /// Thompson Multimedia General Purpose Processor
    pub const TMM_GPP: Self = Self::from_raw(96);
    /// National Semiconductor 32000 series
    pub const NS32K: Self = Self::from_raw(97);
    /// Tenor Network TPC processor
    pub const TPC: Self = Self::from_raw(98);
    /// Trebia SNP 1000 processor
    pub const SNP1K: Self = Self::from_raw(99);
    /// STMicroelectronics ST200 microcontroller
    pub const ST200: Self = Self::from_raw(100);
    /// Ubicom IP2xxx microcontroller family
    pub const IP2K: Self = Self::from_raw(101);
    /// MAX Processor
    pub const MAX: Self = Self::from_raw(102);
    /// National Semiconductor CompactRISC microprocessor
    pub const CR: Self = Self::from_raw(103);
    /// Fujitsu F2MC16
    pub const F2MC16: Self = Self::from_raw(104);
    /// Texas Instruments embedded microcontroller msp430
    pub const MSP430: Self = Self::from_raw(105);
    /// Analog Devices Blackfin (DSP) processor
    pub const BLACKFIN: Self = Self::from_raw(106);
    /// S1C33 Family of Seiko Epson processors
    pub const SE_C33: Self = Self::from_raw(107);
    /// Sharp embedded microprocessor
    pub const SEP: Self = Self::from_raw(108);
    /// Arca RISC Microprocessor
    pub const ARCA: Self = Self::from_raw(109);
    /// Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University
    pub const UNICORE: Self = Self::from_raw(110);
    /// eXcess: 16/32/64-bit configurable embedded CPU
    pub const EXCESS: Self = Self::from_raw(111);
    /// Icera Semiconductor Inc. Deep Execution Processor
    pub const DXP: Self = Self::from_raw(112);
    /// Altera Nios II soft-core processor
    pub const ALTERA_NIOS2: Self = Self::from_raw(113);
    /// National Semiconductor CompactRISC CRX microprocessor
    pub const CRX: Self = Self::from_raw(114);
    /// Motorola XGATE embedded processor
    pub const XGATE: Self = Self::from_raw(115);
    /// Infineon C16x/XC16x processor
    pub const C166: Self = Self::from_raw(116);
    /// Renesas M16C series microprocessors
    pub const M16C: Self = Self::from_raw(117);
    /// Microchip Technology dsPIC30F Digital Signal Controller
    pub const DSPIC30F: Self = Self::from_raw(118);
    /// Freescale Communication Engine RISC core
    pub const CE: Self = Self::from_raw(119);
    /// Renesas M32C series microprocessors
    pub const M32C: Self = Self::from_raw(120);
    /// Altium TSK3000 core
    pub const TSK3000: Self = Self::from_raw(131);
    /// Freescale RS08 embedded processor
    pub const RS08: Self = Self::from_raw(132);
    /// Analog Devices SHARC family of 32-bit DSP processors
    pub const SHARC: Self = Self::from_raw(133);
    /// Cyan Technology eCOG2 microprocessor
    pub const ECOG2: Self = Self::from_raw(134);
    /// Sunplus S+core7 RISC processor
    pub const SCORE7: Self = Self::from_raw(135);
    /// New Japan Radio (NJR) 24-bit DSP Processor
    pub const DSP24: Self = Self::from_raw(136);
    /// Broadcom VideoCore III processor
    pub const VIDEOCORE3: Self = Self::from_raw(137);
    /// RISC processor for Lattice FPGA architecture
    pub const LATTICEMICO32: Self = Self::from_raw(138);
    /// Seiko Epson C17 family
    pub const SE_C17: Self = Self::from_raw(139);
    /// The Texas Instruments TMS320C6000 DSP family
    pub const TI_C6000: Self = Self::from_raw(140);
    /// The Texas Instruments TMS320C2000 DSP family
    pub const TI_C2000: Self = Self::from_raw(141);
    /// The Texas Instruments TMS320C55x DSP family
    pub const TI_C5500: Self = Self::from_raw(142);
    /// Texas Instruments Application Specific RISC Processor, 32bit fetch
    pub const TI_ARP32: Self = Self::from_raw(143);
    /// Texas Instruments Programmable Realtime Unit
    pub const TI_PRU: Self = Self::from_raw(144);
    /// STMicroelectronics 64bit VLIW Data Signal Processor
    pub const MMDSP_PLUS: Self = Self::from_raw(160);
    /// Cypress M8C microprocessor
    pub const CYPRESS_M8C: Self = Self::from_raw(161);
    /// Renesas R32C series microprocessors
    pub const R32C: Self = Self::from_raw(162);
    /// NXP Semiconductors TriMedia architecture family
    pub const TRIMEDIA: Self = Self::from_raw(163);
    /// QUALCOMM DSP6 Processor
    pub const QDSP6: Self = Self::from_raw(164);
    /// Intel 8051 and variants
    pub const _8051: Self = Self::from_raw(165);
    /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
    pub const STXP7X: Self = Self::from_raw(166);
    /// Andes Technology compact code size embedded RISC processor family
    pub const NDS32: Self = Self::from_raw(167);
    /// Cyan Technology eCOG1X family
    pub const ECOG1X: Self = Self::from_raw(168);
    /// Dallas Semiconductor MAXQ30 Core Micro-controllers
    pub const MAXQ30: Self = Self::from_raw(169);
    /// New Japan Radio (NJR) 16-bit DSP Processor
    pub const XIMO16: Self = Self::from_raw(170);
    /// M2000 Reconfigurable RISC Microprocessor
    pub const MANIK: Self = Self::from_raw(171);
    /// Cray Inc. NV2 vector architecture
    pub const CRAYNV2: Self = Self::from_raw(172);
    /// Renesas RX family
    pub const RX: Self = Self::from_raw(173);
    /// Imagination Technologies META processor architecture
    pub const METAG: Self = Self::from_raw(174);
    /// MCST Elbrus general purpose hardware architecture
    pub const MCST_ELBRUS: Self = Self::from_raw(175);
    /// Cyan Technology eCOG16 family
    pub const ECOG16: Self = Self::from_raw(176);
    /// National Semiconductor CompactRISC CR16 16-bit microprocessor
    pub const CR16: Self = Self::from_raw(177);
    /// Freescale Extended Time Processing Unit
    pub const ETPU: Self = Self::from_raw(178);
    /// Infineon Technologies SLE9X core
    pub const SLE9X: Self = Self::from_raw(179);
    /// Intel L10M
    pub const L10M: Self = Self::from_raw(180);
    /// Intel K10M
    pub const K10M: Self = Self::from_raw(181);
    /// ARM 64-bit architecture (AARCH64)
    pub const AARCH64: Self = Self::from_raw(183);
    /// Atmel Corporation 32-bit microprocessor family
    pub const AVR32: Self = Self::from_raw(185);
    /// STMicroeletronics STM8 8-bit microcontroller
    pub const STM8: Self = Self::from_raw(186);
    /// Tilera TILE64 multicore architecture family
    pub const TILE64: Self = Self::from_raw(187);
    /// Tilera TILEPro multicore architecture family
    pub const TILEPRO: Self = Self::from_raw(188);
    /// Xilinx MicroBlaze 32-bit RISC soft processor core
    pub const MICROBLAZE: Self = Self::from_raw(189);
    /// NVIDIA CUDA architecture
    pub const CUDA: Self = Self::from_raw(190);
    /// Tilera TILE-Gx multicore architecture family
    pub const TILEGX: Self = Self::from_raw(191);
    /// CloudShield architecture family
    pub const CLOUDSHIELD: Self = Self::from_raw(192);
    /// KIPO-KAIST Core-A 1st generation processor family
    pub const COREA_1ST: Self = Self::from_raw(193);
    /// KIPO-KAIST Core-A 2nd generation processor family
    pub const COREA_2ND: Self = Self::from_raw(194);
    /// Synopsys ARCompact V2
    pub const ARC_COMPACT2: Self = Self::from_raw(195);
    /// Open8 8-bit RISC soft processor core
    pub const OPEN8: Self = Self::from_raw(196);
    /// Renesas RL78 family
    pub const RL78: Self = Self::from_raw(197);
    /// Broadcom VideoCore V processor
    pub const VIDEOCORE5: Self = Self::from_raw(198);
    /// Renesas 78KOR family
    pub const _78KOR: Self = Self::from_raw(199);
    /// Freescale 56800EX Digital Signal Controller (DSC)
    pub const _56800EX: Self = Self::from_raw(200);
    /// Beyond BA1 CPU architecture
    pub const BA1: Self = Self::from_raw(201);
    /// Beyond BA2 CPU architecture
    pub const BA2: Self = Self::from_raw(202);
    /// XMOS xCORE processor family
    pub const XCORE: Self = Self::from_raw(203);
    /// Microchip 8-bit PIC(r) family
    pub const MCHP_PIC: Self = Self::from_raw(204);
    /// KM211 KM32 32-bit processor
    pub const KM32: Self = Self::from_raw(210);
    /// KM211 KMX32 32-bit processor
    pub const KMX32: Self = Self::from_raw(211);
    /// KM211 KMX16 16-bit processor
    pub const KMX16: Self = Self::from_raw(212);
    /// KM211 KMX8 8-bit processor
    pub const KMX8: Self = Self::from_raw(213);
    /// KM211 KVARC processor
    pub const KVARC: Self = Self::from_raw(214);
    /// Paneve CDP architecture family
    pub const CDP: Self = Self::from_raw(215);
    /// Cognitive Smart Memory Processor
    pub const COGE: Self = Self::from_raw(216);
    /// Bluechip Systems CoolEngine
    pub const COOL: Self = Self::from_raw(217);
    /// Nanoradio Optimized RISC
    pub const NORC: Self = Self::from_raw(218);
    /// CSR Kalimba architecture family
    pub const CSR_KALIMBA: Self = Self::from_raw(219);
    /// Zilog Z80
    pub const Z80: Self = Self::from_raw(220);
    /// Controls and Data Services VISIUMcore processor
    pub const VISIUM: Self = Self::from_raw(221);
    /// FTDI Chip FT32 high performance 32-bit RISC architecture
    pub const FT32: Self = Self::from_raw(222);
    /// Moxie processor family
    pub const MOXIE: Self = Self::from_raw(223);
    /// AMD GPU architecture
    pub const AMDGPU: Self = Self::from_raw(224);
    /// RISC-V
    pub const RISCV: Self = Self::from_raw(243);

    pub const fn from_raw(raw: u16) -> Self {
        Self { inner: raw }
    }
}
