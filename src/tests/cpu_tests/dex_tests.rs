use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0xca_dex() {
    let mut cpu = CPU::new();

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
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![cpu_test_helper::increase_x_by_one(), 0xca, 0x00]);

    assert_eq!(cpu.register_x, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xca_dex_negative_flag() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xca, 0x00]);

    assert_eq!(cpu.register_x, 0xFF);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
