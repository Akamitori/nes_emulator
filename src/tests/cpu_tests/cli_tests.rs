use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0x58_cli() {
    let mut cpu = CPU::new();

    let set_interrupt_flag=0x78;

    cpu.load_and_run(vec![set_interrupt_flag,0x58, 0x00]);

    cpu_test_helper::assert_inactive_interrupt_flag(&cpu);
}