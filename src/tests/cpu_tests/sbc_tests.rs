use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xe9_sbc_immediate() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let accum_value = 80;
    let value = 112;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe9,
        value,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xe9_sbc_immediate_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let accum_value = 10;
    let value = accum_value - 1;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe9, value, 0x00]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe9_sbc_immediate_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let accum_value = 10;
    let value = 20;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe9, value, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xe9_sbc_immediate_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let accum_value = 80;
    let value = (accum_value + 1 as u8).wrapping_neg();

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe9, value, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe9_sbc_immediate_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let accum_value = (48 as u8).wrapping_neg();
    let value = 112;

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe9,
        value,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe5_sbc_zero_page() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe5,
        mem_pos as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xe5_sbc_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = accum_value - 1;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe5, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, 0);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe5_sbc_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = 20;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe5, mem_pos as u8, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xe5_sbc_zero_page_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe5, mem_pos as u8, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe5_sbc_zero_page_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe5,
        mem_pos as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf5_sbc_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xf5,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xf5_sbc_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf5,
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
fn test_0xf5_sbc_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf5,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xf5_sbc_zero_page_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf5,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf5_sbc_zero_page_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xf5,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xed_sbc_absolute() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xed,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xed_sbc_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xed,
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
fn test_0xed_sbc_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xed,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xed_sbc_absolute_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xed,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xed_sbc_absolute_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xed,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xfd_sbc_absolute_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xfd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xfd_sbc_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xfd,
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
fn test_0xfd_sbc_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xfd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xfd_sbc_absolute_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xfd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xfd_sbc_absolute_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xfd,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf9_sbc_absolute_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xf9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xf9_sbc_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf9,
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
fn test_0xf9_sbc_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xf9_sbc_absolute_y_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf9_sbc_absolute_y_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xf9,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe1_sbc_indirect_x() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xe1,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xe1_sbc_indirect_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe1,
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
fn test_0xe1_sbc_indirect_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe1,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xe1_sbc_indirect_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe1,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe1_sbc_indirect_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xe1,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf1_sbc_indirect_y() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        cpu_test_helper::set_carry(),
        0xf1,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xf1_sbc_indirect_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf1,
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
fn test_0xf1_sbc_indirect_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf1,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xf1_sbc_indirect_y_without_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf1,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf1_sbc_indirect_y_with_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value);

    let prep = cpu_test_helper::set_register_a_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xf1,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

// #[test]
// fn test_0xe5_sbc_zero_page_complex() {
//     let bus = Bus::new(test_rom());

//let mut cpu = CPU::new(bus);
//     let minuend : u16=30000;
//     let minuend_bytes=minuend.to_le_bytes();

//     let subtrahend:u16=31000;
//     let subtrahend_bytes=subtrahend.to_le_bytes();


//     let set_low_bytes_of_minuend_to_accumulator = cpu_test_helper::set_register_a_to_value(minuend_bytes[0]);
//     let store_low_bytes_minuend_from_accum_to_memory=cpu_test_helper::store_register_to_address_zero_page(0);
//     let set_high_bytes_of_minuend_to_accumulator = cpu_test_helper::set_register_a_to_value(minuend_bytes[1]);
//     let store_high_bytes_minuend_from_accum_to_memory=cpu_test_helper::store_register_to_address_zero_page(1);

//     let subtrahend_store_memory=2;
//     let set_low_bytes_of_subtrahend_to_accumulator = cpu_test_helper::set_register_a_to_value(subtrahend_bytes[0]);
//     let store_low_bytes_subtrahend_from_accum_to_memory=cpu_test_helper::store_register_to_address_zero_page(subtrahend_store_memory);
//     let set_high_bytes_of_subtrahend_to_accumulator = cpu_test_helper::set_register_a_to_value(subtrahend_bytes[1]);
//     let store_high_bytes_subtrahend_from_accum_to_memory=cpu_test_helper::store_register_to_address_zero_page(subtrahend_store_memory+1);

//     let store_high_bytes_of_result_to_memory=cpu_test_helper::store_register_to_address_zero_page(5);
//     let store_low_bytes_of_result_to_memory=cpu_test_helper::store_register_to_address_zero_page(4);


//     cpu.load_and_run(vec![
//         // Set the stack at it's initial state 
//         set_high_bytes_of_minuend_to_accumulator[0],
//         set_high_bytes_of_minuend_to_accumulator[1],
//         cpu_test_helper::push_accumulator_to_stack(),

//         set_low_bytes_of_minuend_to_accumulator[0],
//         set_low_bytes_of_minuend_to_accumulator[1],
//         cpu_test_helper::push_accumulator_to_stack(),

//         set_high_bytes_of_subtrahend_to_accumulator[0],
//         set_high_bytes_of_subtrahend_to_accumulator[1],
//         cpu_test_helper::push_accumulator_to_stack(),

//         set_low_bytes_of_subtrahend_to_accumulator[0],
//         set_low_bytes_of_subtrahend_to_accumulator[1],
//         cpu_test_helper::push_accumulator_to_stack(),

//         // pull the first subtrahend byte into the stack and store it to memory
//         cpu_test_helper::pull_stack_into_accumulator(), 
//         store_low_bytes_subtrahend_from_accum_to_memory[0],
//         store_low_bytes_subtrahend_from_accum_to_memory[1],

//         // pull the second subtrahend byte into the stack and store it to memory
//         cpu_test_helper::pull_stack_into_accumulator(), 
//         store_high_bytes_subtrahend_from_accum_to_memory[0],
//         store_high_bytes_subtrahend_from_accum_to_memory[1],

//         //pull the low bytes of the minuend into the accumulator
//         cpu_test_helper::pull_stack_into_accumulator(), 
//         cpu_test_helper::set_carry(),

//         0xe5,
//         subtrahend_store_memory,
//         //TAY
//         //PLA
//         //SBC 
//         subtrahend_store_memory+1,

//         store_high_bytes_of_result_to_memory[0], // STORE HIGH BYTES TO MEMORY
//         store_high_bytes_of_result_to_memory[1], // STORE HIGH BYTES TO MEMORY
//         //TYA
//         store_low_bytes_of_result_to_memory[0], // STORE HIGH BYTES TO MEMORY
//         store_low_bytes_of_result_to_memory[1], // STORE HIGH BYTES TO MEMORY
//         0x00,
//     ]);

//     //assert_eq!(cpu.register_a, accum_value.wrapping_sub(value));
//     cpu_test_helper::assert_active_negative_flag(&cpu);
//     cpu_test_helper::assert_inactive_zero_flag(&cpu);
//     cpu_test_helper::assert_inactive_overflow_flag(&cpu);
//     cpu_test_helper::assert_inactive_carry_flag(&cpu);
// }