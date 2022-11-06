use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0xaa_tax_move_a_to_x() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_a_to_value = cpu_test_helper::set_register_a_to_value(10);
    
    cpu.load_and_run(vec![
        set_a_to_value[0],
        set_a_to_value[1],
        0xaa,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_0xaa_tax_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    
    cpu.load_and_run(vec![0xaa, 0x00]);
    
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xaa_tax_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    
    let set_a_to_max = cpu_test_helper::set_register_a_to_value(0xff);
    
    cpu.load_and_run(vec![set_a_to_max[0], set_a_to_max[1], 0xaa, 0x00]);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}