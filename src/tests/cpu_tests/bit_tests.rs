use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0x24_bit_from_memory_zero_page() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x55);

    let prep = cpu_test_helper::set_register_a_to_value(0x55);
    cpu.load_and_run(vec![prep[0], prep[1], 0x24, 0x10, 0x00]);

    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x24_bit_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0b1000_0000);

    cpu.load_and_run(vec![0x24, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x24_bit_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x80);

    let prep = cpu_test_helper::set_register_a_to_value(0);

    cpu.load_and_run(vec![prep[0], prep[1], 0x24, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x24_bit_overflow_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0b0100_0000);

    cpu.load_and_run(vec![0x24, 0x10, 0x00]);

    cpu_test_helper::assert_active_overflow_flag(&cpu);
}

#[test]
fn test_0x2c_bit_from_memory_absolute() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(0x1000, 0x55);

    let prep = cpu_test_helper::set_register_a_to_value(0x55);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x2c,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x2c_bit_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(mem_to_access, 0x80);

    cpu.load_and_run(vec![
        0x2c,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x2c_bit_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(mem_to_access, 0b1000_0000);

    cpu.load_and_run(vec![
        0x2c,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x2c_bit_overflow_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(mem_to_access, 0b0100_0000);

    cpu.load_and_run(vec![
        0x2c,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_overflow_flag(&cpu);
}
