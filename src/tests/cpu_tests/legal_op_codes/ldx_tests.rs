use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xa2_ldx_immediate_load_data() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0x05;
    cpu.load_and_run(vec![0xa2, value, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa2_ldx_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0x00;

    cpu.load_and_run(vec![0xa2, value, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa2_ldx_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0xFF;

    cpu.load_and_run(vec![0xa2, value, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa6_ldx_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa6, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa6_ldx_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa6, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa6_ldx_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![0xa6, mem_to_load as u8, 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xb6_ldx_zero_page_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb6,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb6_ldx_zero_page_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb6,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb6_ldx_zero_page_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb6,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xae_ldx_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xae, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xae_ldx_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xae, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xae_ldx_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    let mem_to_load_bytes = mem_to_load.to_le_bytes();
    cpu.load_and_run(vec![0xae, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xbe_ldx_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x55;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xbe_ldx_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0x00;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xbe_ldx_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x1000;
    let value = 0xFF;
    cpu.mem_write(mem_to_load, value);

    let mem_bytes = (mem_to_load - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
