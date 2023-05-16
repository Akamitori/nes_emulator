use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x4a_lsr_accumulator() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0x50;

    let load_value_to_a = cpu_test_helper::set_register_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4a, 0x00]);

    assert_eq!(cpu.register_a, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x4a_lsr_accumulator_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0x01;

    let load_value_to_a = cpu_test_helper::set_register_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4a, 0x00]);

    assert_eq!(cpu.register_a, value >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x4a_lsr_accumulator_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let value = 0xFF;

    let load_value_to_a = cpu_test_helper::set_register_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4a, 0x00]);

    assert_eq!(cpu.register_a, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x46_lsr_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x46, mem_to_shift as u8, 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x46_lsr_zero_page_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0x01;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x46, mem_to_shift as u8, 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x46_lsr_zero_page_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0xFF;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x46, mem_to_shift as u8, 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x56_lsr_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x56,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x56_lsr_zero_page_x_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0x01;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x56,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x56_lsr_zero_page_x_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x10;
    let value = 0xFF;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x56,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x4e_lsr_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x4e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x4e_lsr_absolute_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0x01;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x4e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x4e_lsr_absolute_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0xFF;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x4e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x5e_lsr_absolute_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x5e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x5e_lsr_absolute_x_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0x01;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x5e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x5e_lsr_absolute_x_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_shift = 0x1000;
    let value = 0xFF;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x5e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, mem_to_shift, value >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}
