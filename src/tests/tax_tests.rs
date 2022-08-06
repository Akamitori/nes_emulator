use crate::{tests::test_helper, CPU};

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    let store_ten_to_register_a = [0xa9, 10];
    cpu.load_and_run(vec![
        store_ten_to_register_a[0],
        store_ten_to_register_a[1],
        0xaa,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xaa, 0x00]);
    test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xaa_tax_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0xFF;
    let set_a_to_max = [0xa9, 0xff];
    cpu.load_and_run(vec![set_a_to_max[0], set_a_to_max[1], 0xaa, 0x00]);
    test_helper::assert_active_negative_flag(&cpu);
}