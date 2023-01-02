use crate::components::cartridge::Rom;

pub struct TestRom {
    pub header: Vec<u8>,
    pub trainer: Option<Vec<u8>>,
    pub pgp_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub fn test_rom(pc_counter_start: u16) -> Rom {
    let prg_rom_banks = 2;
    let chr_rom_banks = 1;
    let mut pgr_rom = vec![0; prg_rom_banks * Rom::PRG_ROM_PAGE_SIZE];
    let pc_counter_start_bytes = pc_counter_start.to_le_bytes();
    pgr_rom[0x7FFC] = pc_counter_start_bytes[0];
    pgr_rom[0x7FFC + 1] = pc_counter_start_bytes[1];
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, // NES^Z string to recognise rom
            prg_rom_banks as u8,    // Number of 16kb ROM banks(prg rom) 
            chr_rom_banks as u8,    // Number of  8kb VROM banks (chr rom)
            0x31,                   // Control byte 1
            0,                     // Control byte 2
            0,                     // Size of PRG RAM in 8k units 
            0,                     // separator 
            0, 0, 0, 0, 0, 0, // Reserved, must be zeroes
        ],
        trainer: None,
        pgp_rom: pgr_rom,
        chr_rom: vec![0; chr_rom_banks * Rom::CHR_ROM_PAGE_SIZE],
    });


    Rom::new(&test_rom).unwrap()
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
