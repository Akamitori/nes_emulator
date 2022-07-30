#[cfg(test)]
mod and_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0x29_and_immediate_and() {
        let mut cpu = CPU::new();

        let prep = test_helper::set_a_to_value(0x32);
        cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0x55, 0x00]);

        assert_eq!(cpu.register_a, 0x32 & 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x29_and_zero_flag() {
        let mut cpu = CPU::new();

        let prep = test_helper::set_a_to_value(0x32);
        cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0x00, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x29_and_negative_flag() {
        let mut cpu = CPU::new();

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0xF5, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x25_and_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        let prep = test_helper::set_a_to_value(0x32);
        cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x32 & 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x25_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xF0);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x25_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x00);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x35_and_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        let prep = test_helper::set_a_to_value(0x11);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x35,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0x11 & 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x35_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x00);

        let prep = test_helper::set_a_to_value(0x11);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x35,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x35_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xF5);

        let prep = test_helper::set_a_to_value(0xF1);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x35,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x2d_and_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        let prep = test_helper::set_a_to_value(0x19);
        cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55 & 0x19);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x2d_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x01);

        let prep = test_helper::set_a_to_value(0x02);
        cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x2d_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xFF);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x3d_and_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        let prep = test_helper::set_a_to_value(0x01);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x3d,
            0xFF,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0x55 & 0x01);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x3d_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x00);

        let prep = test_helper::set_a_to_value(0x11);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x3d,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x3d_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xF9);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x3d,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x39_and_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        let prep = test_helper::set_a_to_value(0x91);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x39,
            0xFF,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0x55 & 0x91);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x39_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x00);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x39,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x39_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xF1);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x39,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x21_and_indirect_x() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;

        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0x55);

        let prep = test_helper::set_a_to_value(0x90);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x21,
            mem_to_load - 1,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0x55 & 0x90);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x21_and_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0);

        let prep = test_helper::set_a_to_value(0xF1);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x21,
            mem_to_load - 1,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x21_and_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0xFF);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x21,
            mem_to_load - 1,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x31_and_indirect_y() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, 0x5);

        let prep = test_helper::set_a_to_value(0x6);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x31,
            mem_to_load,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0x5 & 0x6);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x31_and_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, 0);

        let prep = test_helper::set_a_to_value(0x6);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x31,
            mem_to_load,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x31_and_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, 0xFF);

        let prep = test_helper::set_a_to_value(0xF1);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x31,
            mem_to_load,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }
}
