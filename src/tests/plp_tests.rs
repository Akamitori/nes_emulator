use crate::{tests::test_helper, CPU};

#[test]
fn test_0x28_plp() {
    let mut cpu = CPU::new();

    let value_a = 0xFF;
    let push_accumulator_to_stack=0x48;

    let load_value_a_to_a = test_helper::set_register_a_to_value(value_a);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        push_accumulator_to_stack,
        0x28,
        0x00,
    ]);

    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_interrupt_flag(&cpu);
    test_helper::assert_active_decimal_flag(&cpu);
    test_helper::assert_active_overflow_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_break_1_flag(&cpu);
    test_helper::assert_inactive_break_2_flag(&cpu);
}
