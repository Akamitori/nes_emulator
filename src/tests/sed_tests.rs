use crate::{tests::test_helper, CPU};

#[test]
fn test_0xf8_sed() {
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xf8, 0x00]);

    test_helper::assert_active_decimal_flag(&cpu);
}
