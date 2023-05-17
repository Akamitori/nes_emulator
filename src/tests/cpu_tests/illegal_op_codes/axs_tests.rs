use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xcb_axs_immediate_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 0x80;
    let x_value = 0x81;
    let immediate_value = 0x5;

    let prep_1 = cpu_test_helper::set_accumulator_to_value(accum_value);
    let prep_2 = cpu_test_helper::set_register_x_to_value(x_value);

    cpu.load_and_run(vec![
        prep_1[0],
        prep_1[1],
        prep_2[0],
        prep_2[1],
        0xcb,
        immediate_value,
        0x00,
    ]);

    let result = (x_value & accum_value).wrapping_sub(immediate_value);
    assert_eq!(cpu.register_x, result);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xcb_axs_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 0x80;
    let x_value = 0x81;
    let immediate_value = accum_value & x_value;

    let prep_1 = cpu_test_helper::set_accumulator_to_value(accum_value);
    let prep_2 = cpu_test_helper::set_register_x_to_value(x_value);

    cpu.load_and_run(vec![
        prep_1[0],
        prep_1[1],
        prep_2[0],
        prep_2[1],
        0xcb,
        immediate_value,
        0x00,
    ]);

    let result = (x_value & accum_value).wrapping_sub(immediate_value);
    assert_eq!(cpu.register_x, result);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xcb_axs_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let accum_value = 0x80;
    let x_value = 0x81;
    let immediate_value = (accum_value & x_value) +10;

    let prep_1 = cpu_test_helper::set_accumulator_to_value(accum_value);
    let prep_2 = cpu_test_helper::set_register_x_to_value(x_value);

    cpu.load_and_run(vec![
        prep_1[0],
        prep_1[1],
        prep_2[0],
        prep_2[1],
        0xcb,
        immediate_value,
        0x00,
    ]);

    let result = (x_value & accum_value).wrapping_sub(immediate_value);
    assert_eq!(cpu.register_x, result);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}