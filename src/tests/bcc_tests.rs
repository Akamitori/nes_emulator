use crate::{tests::test_helper, CPU};

#[test]
fn test_0x90_bcc_negative_offset() {
    let mut cpu = CPU::new();
    let accum_value = 0xFF;
    let address = 1;
    let offset = (5 as u8).wrapping_neg();

    let set_a_to_value = test_helper::set_register_a_to_value(accum_value);
    let set_x_to_1 = test_helper::set_register_x_to_value(1);
    let store_x_to_memory = test_helper::store_register_x_to_zero_page(address);
    let add_value_from_address_to_a =
        test_helper::add_with_carry_zero_page_value_to_register_a(address);

    let set_carry = 0x38;
    cpu.load_and_run(vec![
        set_a_to_value[0],
        set_a_to_value[1],
        set_x_to_1[0],
        set_x_to_1[1],
        store_x_to_memory[0],
        store_x_to_memory[1],
        test_helper::clear_carry(),
        add_value_from_address_to_a[0],
        add_value_from_address_to_a[1],
        0x90,
        offset,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
}

#[test]
fn test_0x90_bcc_positive_offset() {
    let mut cpu = CPU::new();
    let accum_value = 20;
    let offset = 2;

    let set_a_to_expected_value = test_helper::set_register_a_to_value(accum_value);
    let set_a_to_a_wrong_value = test_helper::set_register_a_to_value(accum_value * 2);

    let set_carry = 0x38;
    cpu.load_and_run(vec![
        set_a_to_expected_value[0],
        set_a_to_expected_value[1],
        0x90,
        offset,
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value);
}
