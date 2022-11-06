use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;



#[test]
fn test_0x78_sei() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0x78, 0x00]);

    cpu_test_helper::assert_active_interrupt_flag(&cpu);
}
