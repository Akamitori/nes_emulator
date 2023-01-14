use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x6b_arr_immediate() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x0F;
    let accum_value = 0x03;
    let command = 0x6b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], command, value, 0x00]);

    assert_eq!(cpu.register_a, (accum_value & value) >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
}

#[test]
fn test_0x6b_arr_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x01;
    let accum_value = 0x02;
    let command = 0x6b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], command, value, 0x00]);

    assert_eq!(cpu.register_a, (accum_value & value) >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
}

#[test]
fn test_0x6b_arr_immediate_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0b1100_0000;
    let accum_value = 0xFF;
    let command = 0x6b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], command, value, 0x00]);

    assert_eq!(cpu.register_a, (accum_value & value) >> 1);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
}

#[test]
fn test_0x6b_arr_immediate_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0b0100_0000;
    let accum_value = 0xFF;
    let command = 0x6b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], command, value, 0x00]);

    assert_eq!(cpu.register_a, (accum_value & value) >> 1);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
}

#[test]
fn test_0x6b_arr_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x0;
    let accum_value = 0x0;
    let command = 0x6b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![cpu_test_helper::set_carry(), prep[0], prep[1], command, value, 0x00]);

    assert_eq!(cpu.register_a, ((accum_value & value) >> 1) | 0b1000_0000);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
}
