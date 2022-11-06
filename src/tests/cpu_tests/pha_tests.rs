use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0x48_pha() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let memory_of_first_stack_pos = 0x01FD;

    let value_a = 0x30;
    let value_b = 0xF2;

    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = cpu_test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        0x48,
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        0x48,
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&cpu, memory_of_first_stack_pos, value_a);
    cpu_test_helper::assert_address_contains_value(&cpu, memory_of_first_stack_pos - 1, value_b);
}
