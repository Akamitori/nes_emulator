use crate::{tests::test_helper, CPU};

#[test]
fn test_0xa8_tay_move_a_to_y() {
    let mut cpu = CPU::new();
    let set_a_to_value = test_helper::set_register_a_to_value(10);

    cpu.load_and_run(vec![
        set_a_to_value[0],
        set_a_to_value[1],
        0xa8,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 10);
}

#[test]
fn test_0xa8_tay_zero_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa8, 0x00]);

    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xa8_tay_negative_flag() {
    let mut cpu = CPU::new();
    let set_a_to_max = test_helper::set_register_a_to_value(0xff);

    cpu.load_and_run(vec![set_a_to_max[0], set_a_to_max[1], 0xa8, 0x00]);

    test_helper::assert_active_negative_flag(&cpu);
}