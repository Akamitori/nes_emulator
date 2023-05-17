use sdl2::mouse::SystemCursor::No;
use crate::components::bus::Bus;
use crate::components::cartridge::Mirroring;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::ppu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_ppu_vram_read_write() {
    let bus = Bus::new(test_rom(0x0600, None));
    let mut cpu = CPU::new(bus);

    let data_1: u8 = 0x66;
    let data_2: u8 = 0x66;
    let ppu_address_to_write=0x2305;

    let mut codeToRun = vec![];

    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_1, ppu_address_to_write));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_2, ppu_address_to_write+1));
    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write));
    codeToRun.push(0x00);
    
    cpu.load_and_run(codeToRun);

    cpu.mem_read(0x2007);// first read is a dummy read as per emulation
    assert_eq!(cpu.mem_read(0x2007), data_1);
    assert_eq!(cpu.mem_read(0x2007), data_2);
}

#[test]
fn test_ppu_vram_read_write_mirror_down() {
    let bus = Bus::new(test_rom(0x0600, None));
    let mut cpu = CPU::new(bus);

    let data: u8 = 0x66;
    let ppu_address_to_write=0x6305;
    let mirror_down_mask=0x3FFF;
    let ppu_address_to_read= ppu_address_to_write & mirror_down_mask;

    let mut codeToRun = vec![];

    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data, ppu_address_to_write));
    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_read));
    codeToRun.push(0x00);

    cpu.load_and_run(codeToRun);

    cpu.mem_read(0x2007);// first read is a dummy read as per emulation
    assert_eq!(cpu.mem_read(0x2007), data);
}

#[test]
fn test_ppu_vram_read_write_step32() {
    let bus = Bus::new(test_rom(0x0600, None));
    let mut cpu = CPU::new(bus);

    let data_1: u8 = 0x66;
    let data_2= 0x77;
    let data_3= 0x12;
    let ppu_address_to_write=0x21ff;

    let mut codeToRun = vec![];

    codeToRun.extend(ppu_test_helper::write_data_to_PPUCTRL(0b100));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_1, ppu_address_to_write));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_2, ppu_address_to_write+32));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_3, ppu_address_to_write+64));
    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write));
    codeToRun.push(0x00);

    cpu.load_and_run(codeToRun);

    cpu.mem_read(0x2007);// first read is a dummy read as per emulation
    assert_eq!(cpu.mem_read(0x2007), data_1);
    assert_eq!(cpu.mem_read(0x2007), data_2);
    assert_eq!(cpu.mem_read(0x2007), data_3);
}

#[test]
fn test_ppu_vram_read_write_horizontal_mirroring() {
    let bus = Bus::new(test_rom(0x0600, Some(Mirroring::HORIZONTAL)));
    let mut cpu = CPU::new(bus);

    // store results in x;
    let data_1: u8 = 0x66;
    let data_2= 0x77;
    let ppu_address_to_write_1=0x2000;
    let ppu_address_to_write_mirror_1=ppu_address_to_write_1+0x400;
    let ppu_address_to_write_2=0x2800;
    let ppu_address_to_write_mirror_2=ppu_address_to_write_2+0x400;
    let PPUDATA=0x2007;

    let mut codeToRun = vec![];
    
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_1, ppu_address_to_write_1));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_2, ppu_address_to_write_mirror_2));
    
    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write_mirror_1));
    codeToRun.extend(cpu_test_helper::set_register_x_from_memory(PPUDATA));
    codeToRun.extend(cpu_test_helper::set_register_x_from_memory(PPUDATA));

    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write_2));
    codeToRun.extend(cpu_test_helper::set_register_y_from_memory(PPUDATA));
    codeToRun.extend(cpu_test_helper::set_register_y_from_memory(PPUDATA));
    codeToRun.push(0x00);

    cpu.load_and_run(codeToRun);
    
    assert_eq!(cpu.register_x, data_1);
    assert_eq!(cpu.register_y, data_2);
}
#[test]
fn test_ppu_vram_read_write_vertical_mirroring() {
    let bus = Bus::new(test_rom(0x0600, Some(Mirroring::VERTICAL)));
    let mut cpu = CPU::new(bus);

    // store results in x;
    let data_1: u8 = 0x66;
    let data_2= 0x77;
    let ppu_address_to_write_1=0x2000;
    let ppu_address_to_write_mirror_1=ppu_address_to_write_1+0x800;
    let ppu_address_to_write_2=0x2400;
    let ppu_address_to_write_mirror_2=ppu_address_to_write_2+0x800;
    let PPUDATA=0x2007;

    let mut codeToRun = vec![];

    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_1, ppu_address_to_write_1));
    codeToRun.extend(ppu_test_helper::write_data_to_PPUADDRESS(data_2, ppu_address_to_write_mirror_2));

    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write_mirror_1));
    codeToRun.extend(cpu_test_helper::set_register_x_from_memory(PPUDATA));
    codeToRun.extend(cpu_test_helper::set_register_x_from_memory(PPUDATA));

    codeToRun.extend(ppu_test_helper::set_value_to_PPUADDRESS(ppu_address_to_write_2));
    codeToRun.extend(cpu_test_helper::set_register_y_from_memory(PPUDATA));
    codeToRun.extend(cpu_test_helper::set_register_y_from_memory(PPUDATA));
    codeToRun.push(0x00);

    cpu.load_and_run(codeToRun);

    assert_eq!(cpu.register_x, data_1);
    assert_eq!(cpu.register_y, data_2);
}