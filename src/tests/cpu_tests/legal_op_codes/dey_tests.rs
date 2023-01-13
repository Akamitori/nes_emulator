use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x88_dey() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_y_by_one(),
        cpu_test_helper::increase_y_by_one(),
        0x88,
        0x00,
    ]);

    assert_eq!(cpu.register_y, 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x88_dey_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![cpu_test_helper::increase_y_by_one(), 0x88, 0x00]);

    assert_eq!(cpu.register_y, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x88_dey_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![0x88, 0x00]);

    assert_eq!(cpu.register_y, 0xFF);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
