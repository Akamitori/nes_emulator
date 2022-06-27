#[cfg(test)]
mod inc_tests {
    use crate::tests::test_helper::*;
    use crate::*;

    #[test]
    fn test_0xe6_inc_zero_page() {
        let mut cpu = CPU::new();
        let value_to_write=0x05;
        let mem_to_write=0x10;
        cpu.mem_write(mem_to_write, value_to_write);
        
        cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xe6_inc_zero_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xFF;
        let mem_to_write=0x10;
        cpu.mem_write(mem_to_write, value_to_write);

        cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xe6_inc_negative_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xF1;
        let mem_to_write=0x10;
        cpu.mem_write(mem_to_write, value_to_write);

        cpu.load_and_run(vec![0xe6, mem_to_write as u8, 0x00]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xf6_inc_zero_page_x() {
        let mut cpu = CPU::new();
        let value_to_write=0x05;
        let mem_to_write: u16 = 0x10;
        cpu.mem_write(mem_to_write, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xf6,
            mem_to_write as u8 -1,
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_inactive_zero_negative_flags(cpu);
    }

    #[test]
    fn test_0xf6_inc_zero_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xFF;
        let mem_to_write: u16 = 0x10;
        cpu.mem_write(mem_to_write, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xf6,
            mem_to_write as u8 -1,
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xf6_inc_negative_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xF1;
        let mem_to_write: u16 = 0x10;
        cpu.mem_write(mem_to_write, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xf6,
            mem_to_write as u8-1,
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem_to_write),value_to_write.wrapping_add(1));
        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xee_inc_absolute() {
        let mut cpu = CPU::new();
        let value_to_write=0x2;
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        cpu.load_and_run(vec![
            0xee,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_inactive_zero_carry_flags(cpu);
    }

    #[test]
    fn test_0xee_inc_zero_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xFF;
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        cpu.load_and_run(vec![
            0xee,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xee_inc_negative_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xF1;
        let mem: u16 = 0x1000;
        let mem_bytes = mem.to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        cpu.load_and_run(vec![
            0xee,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_active_negative_flag(cpu);
    }

    #[test]
    fn test_0xfe_inc_absolute_x() {
        let mut cpu = CPU::new();
        let value_to_write=0x2;
        let mem: u16 = 0x1000;
        let mem_bytes = (mem-1).to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xfe,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_inactive_zero_carry_flags(cpu);
    }

    #[test]
    fn test_0xfe_inc_zero_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xFF;
        let mem: u16 = 0x1000;
        let mem_bytes = (mem-1).to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xfe,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_active_zero_flag(cpu);
    }

    #[test]
    fn test_0xfe_inc_negative_flag() {
        let mut cpu = CPU::new();
        let value_to_write=0xF1;
        let mem: u16 = 0x1000;
        let mem_bytes = (mem-1).to_le_bytes();
        cpu.mem_write(mem, value_to_write);

        let set_x_to_one = 0xe8;

        cpu.load_and_run(vec![
            set_x_to_one,
            0xfe,
            mem_bytes[0],
            mem_bytes[1],
            0x00,
        ]);

        assert_eq!(cpu.mem_read(mem),value_to_write.wrapping_add(1));
        test_helper::assert_active_negative_flag(cpu);
    }

}