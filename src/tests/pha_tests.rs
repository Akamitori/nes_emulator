use crate::{tests::test_helper, CPU};

#[test]
fn test_0x48_pha() {
    let mut cpu = CPU::new();
    let memory_of_first_stack_pos = 0x01FD;

    let value_a = 0x30;
    let value_b = 0xF2;

    let load_value_a_to_a = test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        0x48,
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        0x48,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, memory_of_first_stack_pos, value_a);
    test_helper::assert_address_contains_value(&cpu, memory_of_first_stack_pos - 1, value_b);
}
