use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x49_eor_immediate() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let a_value = 0x1;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    let immediate_value = 0x0;
    cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, a_value ^ immediate_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x49_eor_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let a_value = 0x32;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    let immediate_value = a_value;

    cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, a_value ^ immediate_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x49_eor_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let a_value = 0xFF;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    let immediate_value = 0x0;
    cpu.load_and_run(vec![prep[0], prep[1], 0x49, immediate_value, 0x00]);

    assert_eq!(cpu.register_a, a_value ^ immediate_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x45_eor_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x10;

    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x45, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x45_eor_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xF0;
    let mem_pos = 0x10;

    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0xF0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x45, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x45_eor_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xF0;
    let mem_pos = 0x10;

    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x01;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0x45, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x55_eor_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x10;
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x55,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, a_value ^ mem_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x55_eor_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x32;
    let mem_pos = 0x10;
    cpu.mem_write(mem_pos, mem_value);

    let a_value = mem_value;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x55,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, a_value ^ mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x55_eor_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xFF;
    let mem_pos = 0x10;
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x55,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, a_value ^ mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x4d_eor_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x4d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x4d_eor_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x1;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x4d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x4d_eor_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xFF;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0x4d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x5d_eor_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x5d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x5d_eor_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = mem_value;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x5d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x5d_eor_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xFF;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x5d,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x59_eor_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x59,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x59_eor_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0x1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = mem_value;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x59,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x59_eor_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_value = 0xF1;
    let mem_pos = 0x1000;
    let mem_pos_bytes = (mem_pos - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_pos, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x59,
        mem_pos_bytes[0],
        mem_pos_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x41_eor_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0x1;
    let mem_indirect = 0x0001;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x41,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x41_eor_indirect_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0x1;
    let mem_indirect = 0x0001;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = mem_value;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x41,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x41_eor_indirect_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0xF1;
    let mem_indirect = 0x0001;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x41,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x51_eor_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0x1;
    let mem_indirect = 0x0011;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect - 1);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x51,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x51_eor_indirect_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0x1;
    let mem_indirect = 0x0011;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect - 1);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = mem_value;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x51,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x51_eor_indirect_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_value = 0xF1;
    let mem_indirect = 0x0011;

    cpu.mem_write_u16(mem_to_load as u16, mem_indirect - 1);
    cpu.mem_write(mem_indirect, mem_value);

    let a_value = 0x0;
    let prep = cpu_test_helper::set_accumulator_to_value(a_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x51,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, mem_value ^ a_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}
