use anyhow::{Result, anyhow, bail};
use elfling::Header64;
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

    println!(
        "  Class:                             {}",
        header.class.as_human_string()
    );

    println!(
        "  Data:                              {}",
        header.encoding.as_human_string()
    );

    println!(
        "  Version:                           {}",
        header.header_version.as_human_string()
    );

    println!(
        "  OS/ABI:                            {}",
        header.os_abi.as_human_string()
    );

    println!(
        "  ABI Version:                       {}",
        header.abi_version
    );

    println!(
        "  Type:                              {}",
        header.object_type.as_human_string()
    );

    println!(
        "  Machine:                           {}",
        header.machine.as_human_string()
    );

    println!(
        "  Version:                           {}",
        header.version.as_human_string()
    );
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
