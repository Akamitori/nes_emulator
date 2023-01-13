use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xa9_lda_immediate_load_data() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0xFF, 0x00]);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa5_lda_from_memory_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa5_lda_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0xFF);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa5_lda_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x00);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb5_from_memory_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load = 0x10;
    cpu.mem_write(mem_to_load, 0x055);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xb5,
        mem_to_load as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb5_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x00);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xb5, 0x0F, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb5_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0xFF);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xb5, 0x0F, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xad_from_memory_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xad_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x00);

    cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xad_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xFF);

    cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xbd_from_memory_absolute_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbd,
        0xFF,
        0x0F,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xbd_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x00);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbd,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xbd_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xFF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xbd,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xb9_from_memory_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb9,
        0xFF,
        0x0F,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb9_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x00);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb9,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb9_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xFF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb9,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa1_from_memory_indirect_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xa1,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa1_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0);

    cpu.load_and_run(vec![0xa1, mem_to_load, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa1_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0xFF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xa1,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xb1_from_memory_indirect_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0x5);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb1,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb1_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;

    cpu.load_and_run(vec![0xb1, mem_to_load, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb1_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0xFF);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb1,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}
