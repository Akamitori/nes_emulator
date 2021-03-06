#[cfg(test)]
mod lda_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x00]);
        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xa5_lda_from_memory_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xa5_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xFF);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xa5_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x00);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xb5_from_memory_zero_page_x() {
        let mut cpu = CPU::new();
        let mem_to_load = 0x10;
        cpu.mem_write(mem_to_load, 0x055);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xb5, mem_to_load as u8 - 1, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xb5_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x00);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xb5, 0x0F, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xb5_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xFF);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xb5, 0x0F, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xad_from_memory_absolute() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xad_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x00);

        cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xad_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xFF);

        cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xbd_from_memory_absolute_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xbd, 0xFF, 0x0F, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xbd_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x00);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xbd, 0xFF, 0x0F, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xbd_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xFF);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xbd, 0xFF, 0x0F, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xb9_from_memory_absolute_y() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x55);

        let set_y_to_one = 0xc8;

        cpu.load_and_run(vec![set_y_to_one, 0xb9, 0xFF, 0x0F, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xb9_zero_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0x00);

        let set_y_to_one = 0xc8;

        cpu.load_and_run(vec![set_y_to_one, 0xb9, 0xFF, 0x0F, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xb9_negative_flag() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1000, 0xFF);

        let set_y_to_one = 0xc8;

        cpu.load_and_run(vec![set_y_to_one, 0xb9, 0xFF, 0x0F, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xa1_from_memory_indirect_x() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0x55);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xa1, mem_to_load - 1, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xa1_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0);

        cpu.load_and_run(vec![0xa1, mem_to_load, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xa1_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0001);
        cpu.mem_write(0x0001, 0xFF);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![set_x_to_one, 0xa1, mem_to_load - 1, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xb1_from_memory_indirect_y() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, 0x5);

        let set_y_to_one = 0xc8;

        cpu.load_and_run(vec![set_y_to_one, 0xb1, mem_to_load, 0x00]);

        assert_eq!(cpu.register_a, 0x5);
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xb1_zero_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;

        cpu.load_and_run(vec![0xb1, mem_to_load, 0x00]);

        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xb1_negative_flag() {
        let mut cpu = CPU::new();
        let mem_to_load: u8 = 0x40;
        cpu.mem_write_u16(mem_to_load as u16, 0x0010);
        cpu.mem_write(0x0011, 0xFF);

        let set_y_to_one = 0xc8;

        cpu.load_and_run(vec![set_y_to_one, 0xb1, mem_to_load, 0x00]);

        test_helper::assert_active_negative_flag(cpu);
    }
}
