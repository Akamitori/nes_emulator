use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x69_adc_immediate() {
    let bus = Bus::new(test_rom(0x0600, None));
    let mut cpu = CPU::new(bus);
    let value = 55;
    let accum_value = 32;

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    cpu.load_and_run(vec![prep[0], prep[1], set_carry, 0x69, value, 0x00]);

    assert_eq!(cpu.register_a, accum_value + value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x69_adc_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x32;
    let negative_value = (value as i8).wrapping_neg() as u8;

    let prep = cpu_test_helper::set_accumulator_to_value(value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x69, negative_value, 0x00]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x69_adc_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 0x32;
    let negative_value = (value as i8).wrapping_neg().wrapping_sub(1) as u8;

    let prep = cpu_test_helper::set_accumulator_to_value(value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x69, negative_value, 0x00]);

    assert_eq!(cpu.register_a as i8, (value + negative_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x69_adc_immediate_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let value = 80;
    let add_that_overflows = 80;

    let prep = cpu_test_helper::set_accumulator_to_value(value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x69, add_that_overflows, 0x00]);

    assert_eq!(cpu.register_a, value + add_that_overflows);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x69_adc_immediate_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));
    let mut cpu = CPU::new(bus);
    let value = 208;
    let add_that_overflows = 144;

    let prep = cpu_test_helper::set_accumulator_to_value(value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x69, add_that_overflows, 0x00]);

    assert_eq!(cpu.register_a, value.wrapping_add(add_that_overflows));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x65_adc_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 55;
    let accum_value = 32;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    cpu.load_and_run(vec![prep[0], prep[1], set_carry, 0x65, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x65_adc_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x65, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x65_adc_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x65, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x65_adc_zero_page_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 80;
    let accum_value = 80;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x65, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x65_adc_zero_page_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 208;
    let accum_value = 144;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x65, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 55;
    let accum_value = 32;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        set_carry,
        0x75,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x75,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x75,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 80;
    let accum_value = 80;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x75,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x75_adc_zero_page_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 208;
    let accum_value = 144;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x75,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x6d_adc_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = 32;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        set_carry,
        0x6d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x6d_adc_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x6d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x6d_adc_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x6d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x6d_adc_absolute_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 80;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x6d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x6d_adc_absolute_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 208;
    let accum_value = 144;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x6d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = 32;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        set_carry,
        cpu_test_helper::increase_x_by_one(),
        0x7d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x7d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x7d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 80;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x7d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x7d_adc_absolute_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 208;
    let accum_value = 144;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x7d,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = 32;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        set_carry,
        0x79,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x79,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x79,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 80;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x79,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x79_adc_absolute_y_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 208;
    let accum_value = 144;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x79,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 55;
    let accum_value = 32;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        set_carry,
        0x61,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x61,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x61,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 80;
    let accum_value = 80;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x61,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x61_adc_indirect_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 208;
    let accum_value = 144;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x61,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 55;
    let accum_value = 32;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let set_carry = 0x38;
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        set_carry,
        0x71,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value + mem_value + 1);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg() as u8;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x71,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 0x32;
    let accum_value = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x71,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a as i8, (mem_value + accum_value) as i8);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y_without_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 80;
    let accum_value = 80;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x71,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value + accum_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x71_adc_indirect_y_with_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 208;
    let accum_value = 144;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x71,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value.wrapping_add(accum_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}
