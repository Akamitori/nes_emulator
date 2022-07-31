use crate::{tests::test_helper, CPU};

#[test]
fn test_0xe8_inx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xFE;
    let set_a_to_negative = [0xa9, 0xf1];
    let transfer_a_to_x = 0xaa;
    cpu.load_and_run(vec![
        set_a_to_negative[0],
        set_a_to_negative[1],
        transfer_a_to_x,
        0xe8,
        0x00,
    ]);

    test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe8_inx_zero_flag() {
    let mut cpu = CPU::new();
    let set_a_to_max = [0xa9, 0xff];
    let transfer_a_to_x = 0xaa;
    cpu.load_and_run(vec![
        set_a_to_max[0],
        set_a_to_max[1],
        transfer_a_to_x,
        0xe8,
        0x00,
    ]);

    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe8_inx_overflow() {
    let mut cpu = CPU::new();
    let set_a_to_max = [0xa9, 0xff];
    let transfer_a_to_x = 0xaa;
    cpu.load_and_run(vec![
        set_a_to_max[0],
        set_a_to_max[1],
        transfer_a_to_x,
        0xe8,
        0xe8,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 1)
}
