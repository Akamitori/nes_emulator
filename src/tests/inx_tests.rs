use crate::{tests::test_helper, CPU};

#[test]
fn test_0xe8_inx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xFE;
    let set_x_to_negative =test_helper::set_register_x_to_value(0xf1);
    cpu.load_and_run(vec![
        set_x_to_negative[0],
        set_x_to_negative[1],
        0xe8,
        0x00,
    ]);

    test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe8_inx_zero_flag() {
    let mut cpu = CPU::new();
    let set_x_to_max = test_helper::set_register_x_to_value(0xff);
    cpu.load_and_run(vec![
        set_x_to_max[0],
        set_x_to_max[1],
        0xe8,
        0x00,
    ]);

    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe8_inx_overflow() {
    let mut cpu = CPU::new();
    let set_x_to_max = test_helper::set_register_x_to_value(0xff);
    cpu.load_and_run(vec![
        set_x_to_max[0],
        set_x_to_max[1],
        0xe8,
        0xe8,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 1);
}
