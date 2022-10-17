use crate::{tests::test_helper, CPU};

#[test]
fn test_0x8a_txa_move_x_to_accumulator() {
    let mut cpu = CPU::new();
    let set_x_to_value = test_helper::set_register_x_to_value(10);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 10);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x8a_txa_zero_flag() {
    let mut cpu = CPU::new();
    let set_x_to_value = test_helper::set_register_x_to_value(0);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 0);
    test_helper::assert_inactive_negative_flag(&cpu);
    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x8a_txa_negative_flag() {
    let mut cpu = CPU::new();
    let  value=(10 as u8).wrapping_neg();
    let set_x_to_value = test_helper::set_register_x_to_value(value);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_flag(&cpu);
}

