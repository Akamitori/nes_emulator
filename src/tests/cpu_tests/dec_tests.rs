use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0xc6_dec_zero_page() {
    let mut cpu = CPU::new();
    let value_to_write = 0x05;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xc6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xc6_dec_zero_flag() {
    let mut cpu = CPU::new();
    let value_to_write = 0x1;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xc6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc6_dec_negative_flag() {
    let mut cpu = CPU::new();
    let value_to_write = 0x0;
    let mem_to_write = 0x10;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![0xc6, mem_to_write as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd6_dec_zero_page_x() {
    let mut cpu = CPU::new();
    let mem_to_write: u16 = 0x10;
    let value_to_write = 0x05;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xd6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0xd6_dec_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_write: u16 = 0x10;
    let value_to_write = 0x01;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xd6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xd6_dec_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_write: u16 = 0x10;
    let value_to_write = 0x0;
    cpu.mem_write(mem_to_write, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xd6,
        mem_to_write as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xce_dec_absolute() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x2;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xce, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xce_dec_zero_flag() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x1;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xce, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xce_dec_negative_flag() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x0;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![0xce, mem_bytes[0], mem_bytes[1], 0x00]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xde_dec_absolute_x() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x2;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xde,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xde_dec_zero_flag() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x1;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xde,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xde_dec_negative_flag() {
    let mut cpu = CPU::new();
    let mem: u16 = 0x1000;
    let value_to_write = 0x0;
    let mem_bytes = (mem - 1).to_le_bytes();
    cpu.mem_write(mem, value_to_write);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0xde,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem), value_to_write.wrapping_sub(1));
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
