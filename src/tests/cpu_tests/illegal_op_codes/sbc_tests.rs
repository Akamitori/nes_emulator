use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xeb_sbc_immediate() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 80;
    let value = 112;

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xeb,
        value,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xeb_sbc_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 10;
    let value = accum_value - 1;

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xeb, value, 0x00]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xeb_sbc_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 10;
    let value = 20;

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xeb, value, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xeb_sbc_immediate_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 80;
    let value = (accum_value + 1 as u8).wrapping_neg();

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xeb, value, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xeb_sbc_immediate_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = (48 as u8).wrapping_neg();
    let value = 112;

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xeb,
        value,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}