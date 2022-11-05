use crate::tests::test_helpers::cpu_test_helper;
use crate::components::cpu::CPU;

#[test]
fn test_0x0a_asl_accumulator() {
    let mut cpu = CPU::new();

    let prep = cpu_test_helper::set_register_a_to_value(0x1);
    cpu.load_and_run(vec![prep[0], prep[1], 0x0a, 0x00]);

    assert_eq!(cpu.register_a, 0x1 << 1);
    cpu_test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x0a_asl_zero_flag() {
    let mut cpu = CPU::new();

    let prep = cpu_test_helper::set_register_a_to_value(0x80);
    cpu.load_and_run(vec![prep[0], prep[1], 0x0a, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x0a_asl_negative_flag() {
    let mut cpu = CPU::new();

    let prep = cpu_test_helper::set_register_a_to_value(0x7F);
    cpu.load_and_run(vec![prep[0], prep[1], 0x0a, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x06_asl_from_memory_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0x06, 0x10, 0x00]);

    assert_eq!(cpu.mem_read(0x10), 0x55 << 1);
}

#[test]
fn test_0x06_asl_negative_flag() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x7F);

    cpu.load_and_run(vec![0x06, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x06_asl_zero_flag() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x80);

    cpu.load_and_run(vec![0x06, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x16_asl_from_memory_zero_page_x() {
    let mut cpu = CPU::new();
    let mem_to_access = 0x10;
    cpu.mem_write(mem_to_access, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x16,
        mem_to_access as u8 - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_access), 0x55 << 1);
}

#[test]
fn test_0x16_asl_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_access: u8 = 0x10;
    cpu.mem_write(mem_to_access as u16, 0x80);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x16,
        mem_to_access - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x16_asl_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_access: u8 = 0x10;
    cpu.mem_write(mem_to_access as u16, 0x7F);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x16,
        mem_to_access - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x0e_asl_from_memory_absolute() {
    let mut cpu = CPU::new();
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(0x1000, 0x55);

    cpu.load_and_run(vec![
        0x0e,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read_u16(mem_to_access), 0x55 << 1);
}

#[test]
fn test_0x0e_asl_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(mem_to_access, 0x80);

    cpu.load_and_run(vec![
        0x0e,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x0e_asl_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_access: u16 = 0x1000;
    let mem_to_access_bytes = mem_to_access.to_le_bytes();
    cpu.mem_write(mem_to_access, 0xFF);

    cpu.load_and_run(vec![
        0x0e,
        mem_to_access_bytes[0],
        mem_to_access_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x1e_asl_from_memory_absolute_x() {
    let mut cpu = CPU::new();
    let mem_to_write = 0x1000;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0x55);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x1e,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_to_write), 0x55 << 1);
}

#[test]
fn test_0x1e_asl_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_write = 0x1000;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0x80);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x1e,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x1e_asl_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_write = 0x1000;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0xF7);

    cpu.load_and_run(vec![
        cpu_test_helper::increase_x_by_one(),
        0x1e,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}
