use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0xe8_inx_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    cpu.register_x = 0xFE;
    let set_x_to_negative =cpu_test_helper::set_register_x_to_value(0xf1);
    cpu.load_and_run(vec![
        set_x_to_negative[0],
        set_x_to_negative[1],
        0xe8,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe8_inx_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_x_to_max = cpu_test_helper::set_register_x_to_value(0xff);
    cpu.load_and_run(vec![
        set_x_to_max[0],
        set_x_to_max[1],
        0xe8,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xe8_inx_overflow() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);
    let set_x_to_max = cpu_test_helper::set_register_x_to_value(0xff);
    cpu.load_and_run(vec![
        set_x_to_max[0],
        set_x_to_max[1],
        0xe8,
        0xe8,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 1);
}
