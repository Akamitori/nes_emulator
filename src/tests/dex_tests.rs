#[cfg(test)]
mod dex_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0xca_dex() {
        let mut cpu = CPU::new();

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, set_x_to_one, 0xca, 0x00]);

        assert_eq!(cpu.register_x, 1);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xca_dex_zero_flag() {
        let mut cpu = CPU::new();

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xca, 0x00]);

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
