use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x4b_alr_accumulator() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x50;
    let immediate_value = 0x51;

    let load_value_to_a = cpu_test_helper::set_accumulator_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4b, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, (value & immediate_value) >> 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}


#[test]
fn test_0x4b_alr_accumulator_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x53;
    let immediate_value = 0x51;

    let load_value_to_a = cpu_test_helper::set_accumulator_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4b, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, (value & immediate_value) >> 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x4b_alr_accumulator_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x01;
    let immediate_value = 0;

    let load_value_to_a = cpu_test_helper::set_accumulator_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4b, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, (value & immediate_value) >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x4b_alr_accumulator_carry_flag_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x01;
    let immediate_value = 0x51;

    let load_value_to_a = cpu_test_helper::set_accumulator_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x4b, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, (value & immediate_value) >> 1);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}