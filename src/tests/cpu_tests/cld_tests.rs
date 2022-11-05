use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0xd8_cld() {
    let mut cpu = CPU::new();

    let set_decimal_flag=0xf8;

    cpu.load_and_run(vec![set_decimal_flag,0xd8, 0x00]);

    cpu_test_helper::assert_inactive_decimal_flag(&cpu);
}
