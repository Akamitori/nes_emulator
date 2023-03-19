use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xa7_lax_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x55;
    cpu.mem_write(mem_pos as u16, mem_value);

    cpu.load_and_run(vec![0xa7, mem_pos, 0x00]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa7_lax_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0xFF;
    cpu.mem_write(mem_pos as u16, mem_value);

    cpu.load_and_run(vec![0xa7, mem_pos, 0x00]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa7_lax_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0;
    cpu.mem_write(mem_pos as u16, mem_value);

    cpu.load_and_run(vec![0xa7, mem_pos, 0x00]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb7_zero_page_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x55;
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb7,
        mem_pos as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb7_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0;
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb7,
        mem_pos as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb7_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0xFF;
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb7,
        mem_pos as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xaf_lax_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0x55;
    let mem_bytes = mem_pos.to_le_bytes();

    cpu.mem_write(mem_pos, mem_value);
    cpu.load_and_run(vec![
        0xaf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xaf_lax_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0;
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    cpu.mem_write(mem_pos, mem_value);
    cpu.load_and_run(vec![
        0xaf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xaf_lax_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0xFF;
    let mem_bytes = mem_pos.to_le_bytes();

    cpu.mem_write(mem_pos, mem_value);
    cpu.load_and_run(vec![
        0xaf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xbf_lax_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0x55;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);


    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xbf_lax_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xbf_lax_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos: u16 = 0x1000;
    let mem_value = 0xFF;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xbf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value);
    assert_eq!(cpu.register_x, mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xa3_lax_indirect_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 1;
    let mem_pos_indirect_value = 0x55;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xa3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xa3_lax_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 1;
    let mem_pos_indirect_value = 0;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xa3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa3_lax_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 1;
    let mem_pos_indirect_value = 0xFF;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xa3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xb3_lax_indirect_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x11;
    let mem_pos_indirect_value = 0x5;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect - 1);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xb3_lax_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x11;
    let mem_pos_indirect_value = 0;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect - 1);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![0xb3, mem_to_load, 0x00]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xb3_lax_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x11;
    let mem_pos_indirect_value = 0xFF;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect - 1);
    cpu.mem_write(mem_pos_indirect, mem_pos_indirect_value);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        0xb3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_pos_indirect_value);
    assert_eq!(cpu.register_x, mem_pos_indirect_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
