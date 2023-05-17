use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xa0_ldy_immediate_load_data() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x05;
    cpu.load_and_run(vec![0xa0, value, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa0_ldy_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x00;

    cpu.load_and_run(vec![0xa0, value, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa0_ldy_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0xFF;

    cpu.load_and_run(vec![0xa0, value, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa4_ldy_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa4, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa4_ldy_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa4, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa4_ldy_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa4, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xb4_ldy_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xb4,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb4_ldy_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xb4,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb4_ldy_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xb4,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xac_ldy_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xac, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xac_ldy_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xac, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xac_ldy_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xac, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xbc_ldy_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbc,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xbc_ldy_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbc,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xbc_ldy_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbc,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
