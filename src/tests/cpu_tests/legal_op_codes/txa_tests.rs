use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x8a_txa_move_x_to_accumulator() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_x_to_value = cpu_test_helper::set_register_x_to_value(10);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 10);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x8a_txa_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let set_x_to_value = cpu_test_helper::set_register_x_to_value(0);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 0);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x8a_txa_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = (10 as u8).wrapping_neg();
    let set_x_to_value = cpu_test_helper::set_register_x_to_value(value);

    cpu.load_and_run(vec![
        set_x_to_value[0],
        set_x_to_value[1],
        0x8a,
        0x00,
    ]);

    assert_eq!(cpu.register_x, value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

