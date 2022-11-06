use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;



#[test]
fn test_0xb8_clv() {
    let mut cpu = CPU::new();

    let value_a = 0xFF;
    let push_accumulator_to_stack=0x48;
    let pull_stack_into_status=0x28;

    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        push_accumulator_to_stack,
        pull_stack_into_status,
        0xb8,
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_interrupt_flag(&cpu);
    cpu_test_helper::assert_active_decimal_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_break_1_flag(&cpu);
    cpu_test_helper::assert_inactive_break_2_flag(&cpu);

    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
}
