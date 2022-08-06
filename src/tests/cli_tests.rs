use crate::{tests::test_helper, CPU};

#[test]
fn test_0x58_cli() {
    let mut cpu = CPU::new();

    let set_interrupt_flag=0x78;

    cpu.load_and_run(vec![set_interrupt_flag,0x58, 0x00]);

    test_helper::assert_inactive_interrupt_flag(&cpu);
}