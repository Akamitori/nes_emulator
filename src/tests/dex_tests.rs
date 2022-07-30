#[cfg(test)]
mod dex_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0xca_dex() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            test_helper::increase_x_by_one(),
            test_helper::increase_x_by_one(),
            0xca,
            0x00,
        ]);

        assert_eq!(cpu.register_x, 1);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xca_dex_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![test_helper::increase_x_by_one(), 0xca, 0x00]);

        assert_eq!(cpu.register_x, 0);
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xca_dex_negative_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xca, 0x00]);

        assert_eq!(cpu.register_x, 0xFF);
        test_helper::assert_active_negative_flag(cpu);
    }
}
