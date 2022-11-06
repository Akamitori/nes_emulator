use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0x68_pla() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    let value_a = 0x30;
    let value_b = 0x12;

    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = cpu_test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    
}

#[test]
fn test_0x68_pla_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    let value_a = 0x30;
    let value_b = 0x0;

    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = cpu_test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    
}

#[test]
fn test_0x68_pla_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    let value_a = 0x30;
    let value_b = 0xFF;

    let load_value_a_to_a = cpu_test_helper::set_register_a_to_value(value_a);
    let load_value_b_to_a = cpu_test_helper::set_register_a_to_value(value_b);

    cpu.load_and_run(vec![
        load_value_a_to_a[0],
        load_value_a_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        load_value_b_to_a[0],
        load_value_b_to_a[1],
        cpu_test_helper::push_accumulator_to_stack(),
        0x68,
        0x00,
    ]);

    assert_eq!(cpu.register_a,value_b);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    
}


