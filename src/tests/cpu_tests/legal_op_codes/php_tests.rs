use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x08_php() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let memory_of_first_stack_pos = 0x01FD;


    let push_accumulator_to_stack = 0x48;
    let pull_stack_into_status = 0x28;

    let value_a = 0b1100_1111;
    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);
    let result = 0xFF;

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        push_accumulator_to_stack,
        pull_stack_into_status,
        0x08,
        0x00,
    ]);

    cpu_test_helper::assert_address_contains_value(&mut cpu, memory_of_first_stack_pos, result);
}
