use crate::{tests::test_helper, CPU};

#[test]
fn test_0x98_tya_move_y_to_accumulator() {
    let mut cpu = CPU::new();
    let set_y_to_value = test_helper::set_register_y_to_value(10);

    cpu.load_and_run(vec![
        set_y_to_value[0],
        set_y_to_value[1],
        0x98,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 10);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x98_tya_zero_flag() {
    let mut cpu = CPU::new();
    let set_y_to_value = test_helper::set_register_y_to_value(0);

    cpu.load_and_run(vec![
        set_y_to_value[0],
        set_y_to_value[1],
        0x98,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 0);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x98_tya_negative_flag() {
    let mut cpu = CPU::new();
    let  value=(10 as u8).wrapping_neg();
    let set_y_to_value = test_helper::set_register_y_to_value(value);

    cpu.load_and_run(vec![
        set_y_to_value[0],
        set_y_to_value[1],
        0x98,
        0x00,
    ]);

    assert_eq!(cpu.register_y, value);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

