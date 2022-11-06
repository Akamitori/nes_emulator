use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0x70_bvs_negative_offset() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let accum_value = 127 as u8;
    let expected_value=(127 as u8).wrapping_neg(); 
    let value_to_add = 1;
    let offset = (4 as u8).wrapping_neg();

    let set_a_to_value = cpu_test_helper::set_register_a_to_value(accum_value);
    let add_value_to_a =cpu_test_helper::add_with_carry_to_register_a(value_to_add);

    let set_carry = 0x38;
    cpu.load_and_run(vec![
        set_a_to_value[0],
        set_a_to_value[1],
        add_value_to_a[0],
        add_value_to_a[1],
        0x70,
        offset,
        0x00,
    ]);
    
    assert_eq!(cpu.register_a, expected_value);
}

#[test]
fn test_0x70_bvs_positive_offset() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let accum_value = 0xFF;
    let  value_to_add=1;
    let offset = 2;

    let set_a_to_expected_value = cpu_test_helper::set_register_a_to_value(accum_value);
    let set_a_to_a_wrong_value = cpu_test_helper::set_register_a_to_value(0);
    let add_value_to_a =cpu_test_helper::add_with_carry_to_register_a(value_to_add);

    cpu.load_and_run(vec![
        set_a_to_expected_value[0],
        set_a_to_expected_value[1],
        add_value_to_a[0],
        add_value_to_a[1],
        0x70,
        offset,
        set_a_to_a_wrong_value[0],
        set_a_to_a_wrong_value[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, (accum_value as u8).wrapping_add(value_to_add));
}
