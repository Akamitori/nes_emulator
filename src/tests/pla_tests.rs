use crate::{tests::test_helper, CPU};

#[test]
fn test_0x68_pla() {
    let mut cpu = CPU::new();

    let value_a = 0x30;
    let value_b = 0x12;

    let load_value_a_to_a = test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    test_helper::assert_inactive_zero_negative_flags(&cpu);
    
}

#[test]
fn test_0x68_pla_zero_flag() {
    let mut cpu = CPU::new();

    let value_a = 0x30;
    let value_b = 0x0;

    let load_value_a_to_a = test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    test_helper::assert_active_zero_flag(&cpu);
    
}

#[test]
fn test_0x68_pla_negative_flag() {
    let mut cpu = CPU::new();

    let value_a = 0x30;
    let value_b = 0xFF;

    let load_value_a_to_a = test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    test_helper::assert_active_negative_flag(&cpu);
    
}


