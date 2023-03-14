use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xc7_dcp_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc7, mem_pos as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xc7_dcp_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc7, mem_pos as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xc7_dcp_zero_page_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 80;
    let accum_value = 85;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc7, mem_pos as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xc7_dcp_zero_page_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0;
    let accum_value = 0;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xc7, mem_pos as u8, 0x00]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd7_dcp_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd7_dcp_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd7_dcp_zero_page_x_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 80;
    let accum_value = 85;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd7_dcp_zero_page_x_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let mem_value = 0;
    let accum_value = 0;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xd7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xcf_dcp_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value: u8 = 55;
    let accum_value = (mem_value.wrapping_sub(1));
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xcf_dcp_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 55;
    let accum_value = (mem_value - 5);
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xcf_dcp_absolute_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 85;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xcf_dcp_absolute_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0;
    let accum_value = 0;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xcf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdf_dcp_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdf_dcp_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xdf_dcp_absolute_x_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 85;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdf_dcp_absolute_x_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0;
    let accum_value = 0;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xdf,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdb_dcp_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xdb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdb_dcp_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xdb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xdb_dcp_absolute_y_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 80;
    let accum_value = 85;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xdb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xdb_dcp_absolute_y_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let mem_value = 0;
    let accum_value = 0;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xdb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xc3_dcp_indirect_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xc3_dcp_indirect_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xc3_dcp_indirect_x_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 80;
    let accum_value = 85;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xc3_dcp_indirect_x_no_flags() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let mem_value = 0;
    let accum_value = 0;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xc3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(mem_pos_indirect), mem_value.wrapping_sub(1));
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd3_dcp_indirect_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value.wrapping_sub(1));

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.mem_read(mem_pos_indirect + 1),
        mem_value.wrapping_sub(1)
    );
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd3_dcp_indirect_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value: u8 = 0x32;
    let accum_value = (mem_value - 5);

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.mem_read(mem_pos_indirect + 1),
        mem_value.wrapping_sub(1)
    );
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xd3_dcp_indirect_y_carry_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 80;
    let accum_value = 85;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.mem_read(mem_pos_indirect + 1),
        mem_value.wrapping_sub(1)
    );
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xd3_dcp_indirect_y_no_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let mem_value = 0;
    let accum_value = 0;

    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xd3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.mem_read(mem_pos_indirect + 1),
        mem_value.wrapping_sub(1)
    );
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}
