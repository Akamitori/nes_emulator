use crate::{tests::test_helper, CPU};

#[test]
fn test_0x78_sei() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0x78, 0x00]);

    test_helper::assert_active_interrupt_flag(&cpu);
}
