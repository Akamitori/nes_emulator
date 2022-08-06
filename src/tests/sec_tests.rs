use crate::{tests::test_helper, CPU};

#[test]
fn test_0x38_sec() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0x38, 0x00]);

    test_helper::assert_active_carry_flag(&cpu);
}
