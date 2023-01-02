use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xe6_inc_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0x05;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xe6_inc_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xFF;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe6_inc_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xF1;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf6_inc_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0x05;
    let mem_to_write: u16 = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xf6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xf6_inc_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xFF;
    let mem_to_write: u16 = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xf6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xf6_inc_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xF1;
    let mem_to_write: u16 = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xf6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xee_inc_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0x2;
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xee, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xee_inc_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xFF;
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xee, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xee_inc_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xF1;
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xee, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xfe_inc_absolute_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0x2;
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xfe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xfe_inc_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xFF;
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xfe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xfe_inc_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value_to_write = 0xF1;
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xfe,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_add(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
