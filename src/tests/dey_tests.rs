#[cfg(test)]
mod dey_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0x88_dey() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            test_helper::increase_y_by_one(),
            test_helper::increase_y_by_one(),
            0x88,
            0x00,
        ]);

        assert_eq!(cpu.register_y, 1);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x88_dey_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![test_helper::increase_y_by_one(), 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0);
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x88_dey_negative_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x88, 0x00]);

        assert_eq!(cpu.register_y, 0xFF);
        test_helper::assert_active_negative_flag(cpu);
    }
}
