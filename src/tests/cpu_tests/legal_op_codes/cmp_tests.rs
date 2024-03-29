use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xc9_cmp_immediate_access() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc9, 0x05, 0x00]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xc9_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc9, 0x05, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc9_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc9, 0x05, 0x00]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xc9_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc9, 0x05, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xc5_cmp_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc5, 0x10, 0x00]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xc5_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc5, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc5_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc5, 0x10, 0x00]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc5_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc5, 0x10, 0x00]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd5_cmp_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_write: u8 = 0x10;
    cpu.mem_write(mem_to_write as u16, 0x05);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd5,
        mem_to_write - 1,
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xd5_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_write: u16 = 0x10;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0x05);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd5,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xd5_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_write: u16 = 0x10;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0x05);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd5,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xd5_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_write: u16 = 0x10;
    let mem_to_write_bytes = (mem_to_write - 1 as u16).to_le_bytes();
    cpu.mem_write(mem_to_write, 0x05);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd5,
        mem_to_write_bytes[0],
        mem_to_write_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xcd_cmp_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xcd_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xcd_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xcd_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = mem.to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xdd_cmp_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xdd_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xdd_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(6);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xdd_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd9_cmp_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xd9_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xd9_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(6);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xd9_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem: u16 = 0x1000;
    let mem_bytes = (mem - 1 as u16).to_le_bytes();
    cpu.mem_write(mem, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xc1_cmp_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc1,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xc1_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc1,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xc1_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc1,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xc1_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0001);
    cpu.mem_write(0x0001, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc1,
        mem_to_load - 1,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd1_cmp_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    cpu.mem_write_u16(mem_to_load as u16, 0x0010);
    cpu.mem_write(0x0011, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(4);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd1,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_inactive_zero_carry_flags(&cpu);
}

#[test]
fn test_0xd1_cmp_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_indirect = 0x0010;
    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect + 1, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd1,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_zero_flag(&cpu);
}

#[test]
fn test_0xd1_cmp_carry_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_indirect = 0x0010;
    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect + 1, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(5);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd1,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_carry_flag(&cpu);
}

#[test]
fn test_0xd1_cmp_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_indirect = 0x0010;
    cpu.mem_write_u16(mem_to_load as u16, mem_indirect);
    cpu.mem_write(mem_indirect + 1, 0x5);

    let prep = cpu_test_helper::set_accumulator_to_value(3);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd1,
        mem_to_load,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
}
