use crate::components::cartridge::Rom;

pub struct TestRom {
    pub header: Vec<u8>,
    pub trainer: Option<Vec<u8>>,
    pub pgp_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, |t| t.len())
            + rom.pgp_rom.len()
            + rom.chr_rom.len(),
    );

    result.extend(&rom.header);

    if let Some(t) = rom.trainer {
        result.extend(t);
    }
    result.extend(&rom.pgp_rom);
    result.extend(&rom.chr_rom);

    result
}

pub fn test_rom() -> Rom {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * Rom::PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * Rom::CHR_ROM_PAGE_SIZE],
    });

    Rom::new(&test_rom).unwrap()
}