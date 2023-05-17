use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x28_plp() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let value_a = 0xFF;
    let push_accumulator_to_stack = 0x48;

    let load_value_a_to_a = cpu_test_helper::set_accumulator_to_value(value_a);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        push_accumulator_to_stack,
        0x28,
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_interrupt_flag(&cpu);
    cpu_test_helper::assert_active_decimal_flag(&cpu);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_break_1_flag(&cpu);
    cpu_test_helper::assert_active_break_2_flag(&cpu);
}
