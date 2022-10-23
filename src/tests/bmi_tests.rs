use crate::{tests::test_helper, CPU};

#[test]
fn test_0x30_bmi_negative_offset() {
    let mut cpu = CPU::new();
    let accum_value = 0xFD;
    let value_to_add = 1;
    let offset = (4 as u8).wrapping_neg();

    let set_a_to_value = test_helper::set_register_a_to_value(accum_value);
    let add_value_to_a =test_helper::add_with_carry_to_register_a(value_to_add);

    cpu.load_and_run(vec![
        set_a_to_value[0],
        set_a_to_value[1],
        add_value_to_a[0],
        add_value_to_a[1],
        0x30,
        offset,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
}

#[test]
fn test_0x30_bmi_positive_offset() {
    let mut cpu = CPU::new();
    let accum_value = 0xFF;
    let offset = 2;

    let set_a_to_expected_value = test_helper::set_register_a_to_value(accum_value);
    let set_a_to_a_wrong_value = test_helper::set_register_a_to_value(accum_value-10);

    cpu.load_and_run(vec![
        set_a_to_expected_value[0],
        set_a_to_expected_value[1],
        0x30,
        offset,
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value);
}