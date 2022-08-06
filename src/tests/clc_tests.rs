use crate::{tests::test_helper, CPU};

#[test]
fn test_0x18_clc() {
    let mut cpu = CPU::new();

    let set_carry_flag=0x38;

    cpu.load_and_run(vec![set_carry_flag,0x18, 0x00]);

    test_helper::assert_inactive_carry_flag(&cpu);
}
