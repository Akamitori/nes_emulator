#[cfg(test)]
mod eor_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0x49_eor_immediate_and() {
        let mut cpu = CPU::new();

        let a_value = 0x1;
        let prep = test_helper::set_a_to_value(a_value);
        let immediate_value = 0x0;
        cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

        assert_eq!(cpu.register_a, a_value ^ immediate_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x49_eor_zero_flag() {
        let mut cpu = CPU::new();

        let value = 0x32;
        let prep = test_helper::set_a_to_value(value);
        let immediate_value = value;

        cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x49_eor_negative_flag() {
        let mut cpu = CPU::new();

        let value = 0xFF;
        let prep = test_helper::set_a_to_value(value);
        let immediate_value = 0x0;
        cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x45_eor_zero_page() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;

        cpu.mem_write(0x10, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x45, 0x10, 0x00]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x45_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xF0;

        cpu.mem_write(0x10, mem_value);

        let a_value = 0x01;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x45, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x45_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xF0;

        cpu.mem_write(0x10, mem_value);

        let a_value = 0xF0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x45, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x55_eor_zero_page_x() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x10, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x55,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, a_value ^ mem_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x55_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0x32;
        cpu.mem_write(0x10, mem_value);

        let a_value = mem_value;
        let prep = test_helper::set_a_to_value(a_value);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x55,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x55_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xFF;
        cpu.mem_write(0x10, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);

        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x55,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x4d_eor_absolute() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x4d, 0x00, 0x10, 0x00]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x4d_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x1;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x4d, 0x00, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x4d_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xFF;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![prep[0], prep[1], 0x4d, 0x00, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x5d_eor_absolute_x() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x5d,
            0xFF,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x5d_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = mem_value;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x5d,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x5d_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xFF;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x5d,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x59_eor_absolute_y() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x59,
            0xFF,
            0x0F,
            0x00,
        ]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x59_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0x1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = mem_value;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x59,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x59_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_value = 0xF1;
        cpu.mem_write(0x1000, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x59,
            0xFF,
            0x0F,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x41_eor_indirect_x() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0x1;

        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x41,
            mem_to_load - 1,
            0x00,
        ]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x41_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0x1;

        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, mem_value);

        let a_value = mem_value;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x41,
            mem_to_load - 1,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x41_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0xF1;

        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_x_by_one(),
            0x41,
            mem_to_load - 1,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0x51_eor_indirect_y() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0x1;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x51,
            mem_to_load,
            0x00,
        ]);

        assert_eq!(cpu.register_a, mem_value ^ a_value);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0x51_eor_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0x1;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, mem_value);

        let a_value = mem_value;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x51,
            mem_to_load,
            0x00,
        ]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0x51_eor_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        let mem_value = 0xF1;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, mem_value);

        let a_value = 0x0;
        let prep = test_helper::set_a_to_value(a_value);
        cpu.load_and_run(vec![
            prep[0],
            prep[1],
            test_helper::increase_y_by_one(),
            0x51,
            mem_to_load,
            0x00,
        ]);

        test_helper::assert_active_negative_flag(cpu);
    }
}
