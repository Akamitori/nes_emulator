use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x0b_anc_immediate() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x03;
    let accum_value = 0x01;
    let command=0x0b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x0b_anc_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x02;
    let accum_value = 0x01;
    let command=0x0b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x0b_anc_immediate_carry_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0xF2;
    let accum_value = 0xF1;
    let command=0x0b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x2b_anc_immediate() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x03;
    let accum_value = 0x01;
    let command=0x2b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x2b_anc_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0x02;
    let accum_value = 0x01;
    let command=0x2b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x2b_anc_immediate_carry_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));
    let mut cpu = CPU::new(bus);
    let value = 0xF2;
    let accum_value = 0xF1;
    let command=0x2b;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1],command, value, 0x00]);

    assert_eq!(cpu.register_a,accum_value & value);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}