use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::components::cartridge::Rom;
use crate::components::bus::Bus;



#[test]
fn test_0xca_dex() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::increase_x_by_one(),
        0xca,
        0x00,
    ]);

    assert_eq!(cpu.register_x, 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xca_dex_zero_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xca_dex_negative_flag() {
    let bus = Bus::new(test_rom());

let mut cpu = CPU::new(bus);

    cpu.load_and_run(vec![0xca, 0x00]);

    assert_eq!(cpu.register_x, 0xFF);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
