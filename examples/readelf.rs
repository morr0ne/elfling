use anyhow::{Result, anyhow, bail};
use elfling::{Class, Encoding, Header64, HeaderVersion, Machine, ObjectType, OsAbi, Version};
use std::fs::File;
use zerocopy::FromBytes;

fn main() -> Result<()> {
    let path = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow!("Please provide a file path"))?;

    let elf_file = File::open(path)?;

    let header = Header64::read_from_io(elf_file)?;

    if !header.magic.is_valid() {
        bail!("Invalid magic!")
    }

    println!("ELF Header:");

    let class = match header.class {
        Class::ELF32 => "ELF32",
        Class::ELF64 => "ELF64",
        _ => "Invalid",
    };
    println!("  Class:                             {class}");

    let data = match header.encoding {
        Encoding::ELFDATA2LSB => "2's complement, little endian",
        Encoding::ELFDATA2MSB => "2's complement, big endian",
        _ => "Invalid",
    };
    println!("  Data:                              {data}");

    let version = match header.header_version {
        HeaderVersion::CURRENT => "1 (current)",
        HeaderVersion::NONE => "0 (invalid)",
        _ => "<unknown>",
    };
    println!("  Version:                           {version}");

    let osabi = match header.os_abi {
        OsAbi::NONE => "UNIX - System V",
        OsAbi::HPUX => "HP-UX",
        OsAbi::NETBSD => "NetBSD",
        OsAbi::LINUX => "Linux",
        OsAbi::SOLARIS => "Solaris",
        OsAbi::AIX => "AIX",
        OsAbi::IRIX => "IRIX",
        OsAbi::FREEBSD => "FreeBSD",
        OsAbi::TRU64 => "TRU64",
        OsAbi::MODESTO => "Modesto",
        OsAbi::OPENBSD => "OpenBSD",
        OsAbi::OPENVMS => "OpenVMS",
        OsAbi::NSK => "NSK",
        OsAbi::AROS => "AROS",
        OsAbi::FENIXOS => "FenixOS",
        OsAbi::CLOUDABI => "CloudABI",
        OsAbi::OPENVOS => "OpenVOS",
        _ => "<unknown>",
    };
    println!("  OS/ABI:                            {osabi}");

    println!(
        "  ABI Version:                       {}",
        header.abi_version
    );

    let file_type = match header.object_type {
        ObjectType::NONE => "NONE (No file type)",
        ObjectType::REL => "REL (Relocatable file)",
        ObjectType::EXEC => "EXEC (Executable file)",
        ObjectType::DYN => "DYN (Shared object file)",
        ObjectType::CORE => "CORE (Core file)",
        _ => "Unknown",
    };
    println!("  Type:                              {file_type}");

    let machine = match header.machine {
        Machine::NONE => "No machine",
        Machine::M32 => "AT&T WE 32100",
        Machine::SPARC => "SPARC",
        Machine::_386 => "Intel 80386",
        Machine::_68K => "Motorola 68000",
        Machine::_88K => "Motorola 88000",
        Machine::IAMCU => "Intel MCU",
        Machine::_860 => "Intel 80860",
        Machine::MIPS => "MIPS I Architecture",
        Machine::S370 => "IBM System/370 Processor",
        Machine::MIPS_RS3_LE => "MIPS RS3000 Little-endian",
        Machine::PARISC => "Hewlett-Packard PA-RISC",
        Machine::VPP500 => "Fujitsu VPP500",
        Machine::SPARC32PLUS => "Enhanced instruction set SPARC",
        Machine::_960 => "Intel 80960",
        Machine::PPC => "PowerPC",
        Machine::PPC64 => "64-bit PowerPC",
        Machine::S390 => "IBM System/390 Processor",
        Machine::SPU => "IBM SPU/SPC",
        Machine::V800 => "NEC V800",
        Machine::FR20 => "Fujitsu FR20",
        Machine::RH32 => "TRW RH-32",
        Machine::RCE => "Motorola RCE",
        Machine::ARM => "ARM 32-bit architecture (AARCH32)",
        Machine::ALPHA => "Digital Alpha",
        Machine::SH => "Hitachi SH",
        Machine::SPARCV9 => "SPARC Version 9",
        Machine::TRICORE => "Siemens TriCore embedded processor",
        Machine::ARC => "Argonaut RISC Core",
        Machine::H8_300 => "Hitachi H8/300",
        Machine::H8_300H => "Hitachi H8/300H",
        Machine::H8S => "Hitachi H8S",
        Machine::H8_500 => "Hitachi H8/500",
        Machine::IA_64 => "Intel IA-64 processor architecture",
        Machine::MIPS_X => "Stanford MIPS-X",
        Machine::COLDFIRE => "Motorola ColdFire",
        Machine::_68HC12 => "Motorola M68HC12",
        Machine::MMA => "Fujitsu MMA Multimedia Accelerator",
        Machine::PCP => "Siemens PCP",
        Machine::NCPU => "Sony nCPU embedded RISC processor",
        Machine::NDR1 => "Denso NDR1 microprocessor",
        Machine::STARCORE => "Motorola Star*Core processor",
        Machine::ME16 => "Toyota ME16 processor",
        Machine::ST100 => "STMicroelectronics ST100 processor",
        Machine::TINYJ => "Advanced Logic Corp. TinyJ embedded processor family",
        Machine::X86_64 => "AMD x86-64 architecture",
        Machine::PDSP => "Sony DSP Processor",
        Machine::PDP10 => "Digital Equipment Corp. PDP-10",
        Machine::PDP11 => "Digital Equipment Corp. PDP-11",
        Machine::FX66 => "Siemens FX66 microcontroller",
        Machine::ST9PLUS => "STMicroelectronics ST9+ 8/16 bit microcontroller",
        Machine::ST7 => "STMicroelectronics ST7 8-bit microcontroller",
        Machine::_68HC16 => "Motorola MC68HC16 Microcontroller",
        Machine::_68HC11 => "Motorola MC68HC11 Microcontroller",
        Machine::_68HC08 => "Motorola MC68HC08 Microcontroller",
        Machine::_68HC05 => "Motorola MC68HC05 Microcontroller",
        Machine::SVX => "Silicon Graphics SVx",
        Machine::ST19 => "STMicroelectronics ST19 8-bit microcontroller",
        Machine::VAX => "Digital VAX",
        Machine::CRIS => "Axis Communications 32-bit embedded processor",
        Machine::JAVELIN => "Infineon Technologies 32-bit embedded processor",
        Machine::FIREPATH => "Element 14 64-bit DSP Processor",
        Machine::ZSP => "LSI Logic 16-bit DSP Processor",
        Machine::MMIX => "Donald Knuth's educational 64-bit processor",
        Machine::HUANY => "Harvard University machine-independent object files",
        Machine::PRISM => "SiTera Prism",
        Machine::AVR => "Atmel AVR 8-bit microcontroller",
        Machine::FR30 => "Fujitsu FR30",
        Machine::D10V => "Mitsubishi D10V",
        Machine::D30V => "Mitsubishi D30V",
        Machine::V850 => "NEC v850",
        Machine::M32R => "Mitsubishi M32R",
        Machine::MN10300 => "Matsushita MN10300",
        Machine::MN10200 => "Matsushita MN10200",
        Machine::PJ => "picoJava",
        Machine::OPENRISC => "OpenRISC 32-bit embedded processor",
        Machine::ARC_COMPACT => "ARC International ARCompact processor",
        Machine::XTENSA => "Tensilica Xtensa Architecture",
        Machine::VIDEOCORE => "Alphamosaic VideoCore processor",
        Machine::TMM_GPP => "Thompson Multimedia General Purpose Processor",
        Machine::NS32K => "National Semiconductor 32000 series",
        Machine::TPC => "Tenor Network TPC processor",
        Machine::SNP1K => "Trebia SNP 1000 processor",
        Machine::ST200 => "STMicroelectronics ST200 microcontroller",
        Machine::IP2K => "Ubicom IP2xxx microcontroller family",
        Machine::MAX => "MAX Processor",
        Machine::CR => "National Semiconductor CompactRISC microprocessor",
        Machine::F2MC16 => "Fujitsu F2MC16",
        Machine::MSP430 => "Texas Instruments embedded microcontroller msp430",
        Machine::BLACKFIN => "Analog Devices Blackfin (DSP) processor",
        Machine::SE_C33 => "S1C33 Family of Seiko Epson processors",
        Machine::SEP => "Sharp embedded microprocessor",
        Machine::ARCA => "Arca RISC Microprocessor",
        Machine::UNICORE => {
            "Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University"
        }
        Machine::EXCESS => "eXcess: 16/32/64-bit configurable embedded CPU",
        Machine::DXP => "Icera Semiconductor Inc. Deep Execution Processor",
        Machine::ALTERA_NIOS2 => "Altera Nios II soft-core processor",
        Machine::CRX => "National Semiconductor CompactRISC CRX microprocessor",
        Machine::XGATE => "Motorola XGATE embedded processor",
        Machine::C166 => "Infineon C16x/XC16x processor",
        Machine::M16C => "Renesas M16C series microprocessors",
        Machine::DSPIC30F => "Microchip Technology dsPIC30F Digital Signal Controller",
        Machine::CE => "Freescale Communication Engine RISC core",
        Machine::M32C => "Renesas M32C series microprocessors",
        Machine::TSK3000 => "Altium TSK3000 core",
        Machine::RS08 => "Freescale RS08 embedded processor",
        Machine::SHARC => "Analog Devices SHARC family of 32-bit DSP processors",
        Machine::ECOG2 => "Cyan Technology eCOG2 microprocessor",
        Machine::SCORE7 => "Sunplus S+core7 RISC processor",
        Machine::DSP24 => "New Japan Radio (NJR) 24-bit DSP Processor",
        Machine::VIDEOCORE3 => "Broadcom VideoCore III processor",
        Machine::LATTICEMICO32 => "RISC processor for Lattice FPGA architecture",
        Machine::SE_C17 => "Seiko Epson C17 family",
        Machine::TI_C6000 => "The Texas Instruments TMS320C6000 DSP family",
        Machine::TI_C2000 => "The Texas Instruments TMS320C2000 DSP family",
        Machine::TI_C5500 => "The Texas Instruments TMS320C55x DSP family",
        Machine::TI_ARP32 => "Texas Instruments Application Specific RISC Processor, 32bit fetch",
        Machine::TI_PRU => "Texas Instruments Programmable Realtime Unit",
        Machine::MMDSP_PLUS => "STMicroelectronics 64bit VLIW Data Signal Processor",
        Machine::CYPRESS_M8C => "Cypress M8C microprocessor",
        Machine::R32C => "Renesas R32C series microprocessors",
        Machine::TRIMEDIA => "NXP Semiconductors TriMedia architecture family",
        Machine::QDSP6 => "QUALCOMM DSP6 Processor",
        Machine::_8051 => "Intel 8051 and variants",
        Machine::STXP7X => {
            "STMicroelectronics STxP7x family of configurable and extensible RISC processors"
        }
        Machine::NDS32 => "Andes Technology compact code size embedded RISC processor family",
        Machine::ECOG1X => "Cyan Technology eCOG1X family",
        Machine::MAXQ30 => "Dallas Semiconductor MAXQ30 Core Micro-controllers",
        Machine::XIMO16 => "New Japan Radio (NJR) 16-bit DSP Processor",
        Machine::MANIK => "M2000 Reconfigurable RISC Microprocessor",
        Machine::CRAYNV2 => "Cray Inc. NV2 vector architecture",
        Machine::RX => "Renesas RX family",
        Machine::METAG => "Imagination Technologies META processor architecture",
        Machine::MCST_ELBRUS => "MCST Elbrus general purpose hardware architecture",
        Machine::ECOG16 => "Cyan Technology eCOG16 family",
        Machine::CR16 => "National Semiconductor CompactRISC CR16 16-bit microprocessor",
        Machine::ETPU => "Freescale Extended Time Processing Unit",
        Machine::SLE9X => "Infineon Technologies SLE9X core",
        Machine::L10M => "Intel L10M",
        Machine::K10M => "Intel K10M",
        Machine::AARCH64 => "ARM 64-bit architecture (AARCH64)",
        Machine::AVR32 => "Atmel Corporation 32-bit microprocessor family",
        Machine::STM8 => "STMicroeletronics STM8 8-bit microcontroller",
        Machine::TILE64 => "Tilera TILE64 multicore architecture family",
        Machine::TILEPRO => "Tilera TILEPro multicore architecture family",
        Machine::MICROBLAZE => "Xilinx MicroBlaze 32-bit RISC soft processor core",
        Machine::CUDA => "NVIDIA CUDA architecture",
        Machine::TILEGX => "Tilera TILE-Gx multicore architecture family",
        Machine::CLOUDSHIELD => "CloudShield architecture family",
        Machine::COREA_1ST => "KIPO-KAIST Core-A 1st generation processor family",
        Machine::COREA_2ND => "KIPO-KAIST Core-A 2nd generation processor family",
        Machine::ARC_COMPACT2 => "Synopsys ARCompact V2",
        Machine::OPEN8 => "Open8 8-bit RISC soft processor core",
        Machine::RL78 => "Renesas RL78 family",
        Machine::VIDEOCORE5 => "Broadcom VideoCore V processor",
        Machine::_78KOR => "Renesas 78KOR family",
        Machine::_56800EX => "Freescale 56800EX Digital Signal Controller (DSC)",
        Machine::BA1 => "Beyond BA1 CPU architecture",
        Machine::BA2 => "Beyond BA2 CPU architecture",
        Machine::XCORE => "XMOS xCORE processor family",
        Machine::MCHP_PIC => "Microchip 8-bit PIC(r) family",
        Machine::KM32 => "KM211 KM32 32-bit processor",
        Machine::KMX32 => "KM211 KMX32 32-bit processor",
        Machine::KMX16 => "KM211 KMX16 16-bit processor",
        Machine::KMX8 => "KM211 KMX8 8-bit processor",
        Machine::KVARC => "KM211 KVARC processor",
        Machine::CDP => "Paneve CDP architecture family",
        Machine::COGE => "Cognitive Smart Memory Processor",
        Machine::COOL => "Bluechip Systems CoolEngine",
        Machine::NORC => "Nanoradio Optimized RISC",
        Machine::CSR_KALIMBA => "CSR Kalimba architecture family",
        Machine::Z80 => "Zilog Z80",
        Machine::VISIUM => "Controls and Data Services VISIUMcore processor",
        Machine::FT32 => "FTDI Chip FT32 high performance 32-bit RISC architecture",
        Machine::MOXIE => "Moxie processor family",
        Machine::AMDGPU => "AMD GPU architecture",
        Machine::RISCV => "RISC-V",
        _ => "<unknown>",
    };
    println!("  Machine:                           {machine}");

    let obj_version = match header.version {
        Version::CURRENT => "0x1",
        Version::NONE => "0x0 (invalid)",
        _ => "<unknown>",
    };
    println!("  Version:                           {obj_version}");
    println!(
        "  Entry point address:               0x{:x}",
        header.entry_point
    );
    println!(
        "  Start of program headers:          {}",
        header.program_header_offset
    );
    println!(
        "  Start of section headers:          {}",
        header.section_header_offset
    );
    println!("  Flags:                             0x{:x}", header.flags);
    println!(
        "  Size of this header:               {}",
        header.header_size
    );
    println!(
        "  Size of program headers:           {}",
        header.program_header_entry_size
    );
    println!(
        "  Number of program headers:         {}",
        header.program_header_count
    );
    println!(
        "  Size of section headers:           {}",
        header.section_header_entry_size
    );
    println!(
        "  Number of section headers:         {}",
        header.section_header_count
    );
    println!(
        "  Section header string table index: {}",
        header.section_header_string_table_index
    );

    Ok(())
}
