use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0x4c_jmp_absolute() {
    let mut cpu = CPU::new();
    let accum_value = 0xFF;
    let initial_program_counter = 0x8000;
    let address = (initial_program_counter + 7 as u16).to_le_bytes();

    let set_a_to_expected_value = cpu_test_helper::set_register_a_to_value(accum_value);
    let set_a_to_a_wrong_value = cpu_test_helper::set_register_a_to_value(0);

    cpu.load_and_run(vec![
        set_a_to_expected_value[0],
        set_a_to_expected_value[1],
        0x4c,
        address[0],
        address[1],
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value);
}

#[test]
fn test_0x6c_jmp_indirect() {
    let mut cpu = CPU::new();
    let accum_value = 0xFF;
    let initial_program_counter = 0x8000;
    let address_pointing_to_address: u16 = 0;
    let address_pointer = address_pointing_to_address.to_le_bytes();
    let address = (initial_program_counter + 7 as u16).to_le_bytes();

    cpu.mem_write(address_pointing_to_address, address[0]);
    cpu.mem_write(address_pointing_to_address + 1, address[1]);

    let set_a_to_expected_value = cpu_test_helper::set_register_a_to_value(accum_value);
    let set_a_to_a_wrong_value = cpu_test_helper::set_register_a_to_value(0);

    cpu.load_and_run(vec![
        set_a_to_expected_value[0],
        set_a_to_expected_value[1],
        0x6c,
        address_pointer[0],
        address_pointer[1],
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value);
}

#[test]
fn test_0x6c_jmp_indirect_page_boundary() {
    let mut cpu = CPU::new();
    let accum_initial_value = 0xFF;
    let accum_final_value = 0xF1;
    let initial_program_counter = 0x8000;
    let address_pointing_to_address = 0x10FF;
    let address_pointing_to_address_lsb: u16 = address_pointing_to_address;
    let address_pointing_to_address_msb: u16 = address_pointing_to_address & 0xFF00;
    let address_pointer = address_pointing_to_address.to_le_bytes();
    let address = (initial_program_counter + 7 as u16).to_le_bytes();

    cpu.mem_write(address_pointing_to_address_lsb, address[0]);
    cpu.mem_write(address_pointing_to_address_msb, address[1]);

    let set_a_to_initial_value = cpu_test_helper::set_register_a_to_value(accum_initial_value);
    let set_a_to_final_value = cpu_test_helper::set_register_a_to_value(accum_final_value);
    let set_a_to_a_wrong_value = cpu_test_helper::set_register_a_to_value(0);

    cpu.load_and_run(vec![
        set_a_to_initial_value[0],
        set_a_to_initial_value[1],
        0x6c,
        address_pointer[0],
        address_pointer[1],
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        set_a_to_final_value[0],
        set_a_to_final_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_final_value);
}
