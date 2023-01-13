use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xc8_iny_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    cpu.register_y = 0xFE;
    let set_y_to_negative = cpu_test_helper::set_register_y_to_value(0xf1);

    cpu.load_and_run(vec![
        set_y_to_negative[0],
        set_y_to_negative[1],
        0xc8,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xc8_iny_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let set_y_to_max = cpu_test_helper::set_register_y_to_value(0xff);

    cpu.load_and_run(vec![
        set_y_to_max[0],
        set_y_to_max[1],
        0xc8,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc8_iny_overflow() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let set_y_to_max = cpu_test_helper::set_register_y_to_value(0xff);

    cpu.load_and_run(vec![
        set_y_to_max[0],
        set_y_to_max[1],
        0xc8,
        0xc8,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 1)
}
