use crate::components::cartridge::{Mirroring, Rom};
use crate::tests::test_helpers::rom_test_helper::TestRom;
use crate::tests::test_helpers::rom_test_helper::create_rom;


#[test]
fn load_rom_dump() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * Rom::PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * Rom::CHR_ROM_PAGE_SIZE],
    });

    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * Rom::CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * Rom::PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
}

#[test]
fn load_rom_dump_with_trainer_ignores_trainer() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            0x02,
            0x01,
            0x31 | 0b100,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
        ],
        trainer: Some(vec![0; 512]),
        pgp_rom: vec![1; 2 * Rom::PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * Rom::CHR_ROM_PAGE_SIZE],
    });

    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * Rom::CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * Rom::PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
}

#[test]
fn load_rom_dump_nes2_is_not_supported() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x31, 0x8, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 1 * Rom::PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * Rom::CHR_ROM_PAGE_SIZE],
    });
    let rom = Rom::new(&test_rom);
    match rom {
        Result::Ok(_) => assert!(false, "should not load rom"),
        Result::Err(str) => assert_eq!(str, "NES2.0 format is not supported"),
    }
}

#[test]
fn load_rom_dump_file_not_ines_not_supported() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x10, 0x01, 0x01, 0x31, 0x8, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 1 * Rom::PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * Rom::CHR_ROM_PAGE_SIZE],
    });
    let rom = Rom::new(&test_rom);
    match rom {
        Result::Ok(_) => assert!(false, "should not load rom"),
        Result::Err(str) => assert_eq!(str, "File is not in iNES file format"),
    }
}