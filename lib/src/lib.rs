use anyhow::{anyhow, Result};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::{io::Cursor, str::from_utf8};

pub mod equipment;
pub mod stats;

pub fn validate_sram(sram: &[u8]) -> Result<()> {
    let mut cur = Cursor::new(sram);

    // Check the length. 32768 bytes as of v30.0.4, 2019-11-15 ROM build
    if sram.len() != 32768 {
        return Err(anyhow!("Validation Error: Unexpected file size"));
    }
    // Check the checksum validity value and the rando-specific file marker
    // 0x55AA and 0xFF
    let checksum_validity: u16 = LittleEndian::read_u16(&sram[0x3E1..0x3E3]);
    if checksum_validity != 0x55AA || sram[0x4F0] != 0xFF {
        return Err(anyhow!("Validation Error: Invalid file"));
    }

    // Check the first two characters of the rom name for VT or ER
    let valid_names: [&str; 2] = ["VT", "ER"];
    let rom_name = &sram[0x2000..0x2002];
    if valid_names
        .iter()
        .any(|&x| x == from_utf8(&rom_name).unwrap())
        == false
    {
        return Err(anyhow!("Validation Error: Invalid ROM name"));
    }

    // Now we check the SRAM's own "inverse" checksum
    let mut checksum = 0u16;
    cur.set_position(0x00);
    while cur.position() < 0x4FE {
        let bytes = cur.read_u16::<LittleEndian>()?;
        checksum = checksum.overflowing_add(bytes).0;
    }
    let expected_inv_checksum = 0x5A5Au16.overflowing_sub(checksum).0;
    let inv_checksum: u16 = LittleEndian::read_u16(&sram[0x4FE..0x500]);
    if inv_checksum != expected_inv_checksum {
        return Err(anyhow!("Validation Error: Invalid checksum"));
    }

    return Ok(());
}

pub(crate) fn bitmask(bits: u32) -> u32 {
    (1u32 << bits) - 1u32
}
