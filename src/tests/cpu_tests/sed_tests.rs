use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;



#[test]
fn test_0xf8_sed() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xf8, 0x00]);

    cpu_test_helper::assert_active_decimal_flag(&cpu);
}
