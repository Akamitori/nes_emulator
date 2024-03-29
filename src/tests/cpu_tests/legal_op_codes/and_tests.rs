use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0x29_and_immediate_and() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(0x32);
    cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0x55, 0x00]);

    assert_eq!(cpu.register_a, 0x32 & 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x29_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(0x32);
    cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0x00, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x29_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![prep[0], prep[1], 0x29, 0xF5, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x25_and_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x32);
    cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x32 & 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x25_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0xF0);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x25_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x00);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![prep[0], prep[1], 0x25, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x35_and_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x11);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x35,
        0x0F,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x11 & 0x55);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x35_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x00);

    let prep = cpu_test_helper::set_accumulator_to_value(0x11);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x35,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x35_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0xF5);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x35,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x2d_and_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x19);
    cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55 & 0x19);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x2d_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x01);

    let prep = cpu_test_helper::set_accumulator_to_value(0x02);
    cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x2d_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xFF);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![prep[0], prep[1], 0x2d, 0x00, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x3d_and_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x01);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x3d,
        0xFF,
        0x0F,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55 & 0x01);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x3d_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x00);

    let prep = cpu_test_helper::set_accumulator_to_value(0x11);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x3d,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x3d_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xF9);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x3d,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x39_and_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x91);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x39,
        0xFF,
        0x0F,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55 & 0x91);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x39_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0x00);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x39,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x39_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x1000, 0xF1);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x39,
        0xFF,
        0x0F,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x21_and_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let indirect_address = 0x50;

    cpu.mem_write_u16(mem_to_load as u16, indirect_address);
    cpu.mem_write(indirect_address, 0x55);

    let prep = cpu_test_helper::set_accumulator_to_value(0x90);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x21,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x55 & 0x90);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x21_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let indirect_address = 0x50;
    cpu.mem_write_u16(mem_to_load as u16, indirect_address);
    cpu.mem_write(indirect_address, 0);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x21,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x21_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0xFF);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0x21,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0x31_and_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(0x6);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x31,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0x5 & 0x6);
    cpu_test_helper::assert_inactive_zero_negative_flags(&cpu);
}

#[test]
fn test_0x31_and_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0);

    let prep = cpu_test_helper::set_accumulator_to_value(0x6);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x31,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0x31_and_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0xFF);

    let prep = cpu_test_helper::set_accumulator_to_value(0xF1);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0x31,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}
