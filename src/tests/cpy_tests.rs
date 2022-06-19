#[cfg(test)]
mod cpy_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0xc0_cpy_immediate_access() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xc0, 0x01, 0x00]);

        test_helper::assert_inactive_zero_carry_flags(cpu);
    }

    #[test]
    fn test_0xc0_cpy_zero_flag() {
        let mut cpu = CPU::new();

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc0, 0x01, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xc0_cpy_carry_flag() {
        let mut cpu = CPU::new();

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc0, 0x01, 0x00]);

        test_helper::assert_active_carry_flag(cpu);
    }

    #[test]
    fn test_0xc0_cpy_negative_flag() {
        let mut cpu = CPU::new();

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc0, 0x05, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xc4_cpy_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x1);

        cpu.load_and_run(vec![0xc4, 0x10, 0x00]);

        test_helper::assert_inactive_zero_carry_flags(cpu);
    }

    #[test]
    fn test_0xc4_cpy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x1);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc4, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xc4_cpy_carry_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x1);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc4, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xc4_cpy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x5);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![increase_y_to_one, 0xc4, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xcc_cpy_absolute() {
        let mut cpu = CPU::new();
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, 0x1);

        let prep = test_helper::set_a_to_value(4);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            0xcc,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        test_helper::assert_inactive_zero_carry_flags(cpu);
    }

    #[test]
    fn test_0xcc_cpy_zero_flag() {
        let mut cpu = CPU::new();
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, 0x1);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![
            increase_y_to_one,
            0xcc,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xcc_cpy_carry_flag() {
        let mut cpu = CPU::new();
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, 0x1);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![
            increase_y_to_one,
            0xcc,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        test_helper::assert_active_carry_flag(cpu);
    }

    #[test]
    fn test_0xcc_cpy_negative_flag() {
        let mut cpu = CPU::new();
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, 0x5);

        let increase_y_to_one = 0xc8;
        cpu.load_and_run(vec![
            increase_y_to_one,
            0xcc,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }
}
