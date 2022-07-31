use crate::{tests::test_helper, CPU};

#[test]
fn test_0x2a_rol_accumulator_base() {
    let mut cpu = CPU::new();
    let value = 0x51;

    let load_value_to_a = test_helper::set_a_to_value(value);

    cpu.load_and_run(vec![
        load_value_to_a[0],
        load_value_to_a[1],
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x2a,
        0x00,
    ]);

    assert_eq!(cpu.register_a, value);
    test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x2a_rol_accumulator_carry_flag_zero_flag() {
    let mut cpu = CPU::new();
    let value = 0x80;

    let load_value_to_a = test_helper::set_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x2a, 0x00]);

    assert_eq!(cpu.register_a, value << 1);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x2a_rol_accumulator_carry_flag_negative_flag() {
    let mut cpu = CPU::new();
    let value = 0xF0;

    let load_value_to_a = test_helper::set_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x2a, 0x00]);

    assert_eq!(cpu.register_a, value << 1);
    test_helper::assert_inactive_zero_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x2a_rol_accumulator_negative_flag() {
    let mut cpu = CPU::new();
    let value = 0x50;

    let load_value_to_a = test_helper::set_a_to_value(value);

    cpu.load_and_run(vec![load_value_to_a[0], load_value_to_a[1], 0x2a, 0x00]);

    assert_eq!(cpu.register_a, value << 1);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0x26_rol_zero_page_base() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x51;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x26,
        mem_to_shift as u8,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value);
    test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x26_rol_zero_page_carry_flag_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x80;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x26, mem_to_shift as u8, 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x26_rol_zero_page_carry_flag_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0xF0;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x26, mem_to_shift as u8, 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_inactive_zero_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x26_rol_zero_page_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![0x26, mem_to_shift as u8, 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0x36_rol_zero_page_x_base() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x51;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x36,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value);
    test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x36_rol_zero_page_x_carry_flag_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x80;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x36,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x36_rol_zero_page_x_carry_flag_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0xF0;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x36,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_inactive_zero_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x36_rol_zero_page_x_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x10;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x36,
        mem_to_shift as u8 - 1,
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0x2e_rol_absolute_base() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x51;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x2e,
        mem_to_load_bytes[0],
        mem_to_load_bytes[1],
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value);
    test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x2e_rol_absolute_carry_flag_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x80;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x2e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x2e_rol_absolute_carry_flag_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0xF0;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x2e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_inactive_zero_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x2e_rol_absolute_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);

    let mem_to_load_bytes = mem_to_shift.to_le_bytes();
    cpu.load_and_run(vec![0x2e, mem_to_load_bytes[0], mem_to_load_bytes[1], 0x00]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0x3e_rol_absolute_x_base() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x00
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value);
    test_helper::assert_inactive_zero_negative_carry_flag(&cpu);
}

#[test]
fn test_0x3e_rol_absolute_x_carry_flag_zero_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x80;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);
    
    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_zero_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
    test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x3e_rol_absolute_x_carry_flag_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0xF0;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_inactive_zero_flag(&cpu);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0x3e_rol_absolute_x_negative_flag() {
    let mut cpu = CPU::new();
    let mem_to_shift = 0x1000;
    let value = 0x50;

    cpu.mem_write(mem_to_shift, value);
    let mem_bytes = (mem_to_shift - 1).to_le_bytes();
    cpu.load_and_run(vec![
        test_helper::increase_x_by_one(),
        0x3e,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    test_helper::assert_address_contains_value(&cpu, mem_to_shift, value << 1);
    test_helper::assert_active_negative_flag(&cpu);
    test_helper::assert_inactive_zero_carry_flags(&cpu);
}
