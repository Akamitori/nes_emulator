use crate::components::bus::Bus;
use crate::components::cartridge::Rom;
use crate::components::cpu::CPU;
use crate::components::mem::Mem;
use crate::tests::test_helpers::cpu_test_helper;
use crate::tests::test_helpers::rom_test_helper::test_rom;

#[test]
fn test_0xe7_sbc_zero_page() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe7,
        mem_pos as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xe7_sbc_zero_page_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = accum_value - 1;

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe7, mem_pos as u8, 0x00]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe7_sbc_zero_page_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = 20;

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe7, mem_pos as u8, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xe7_sbc_zero_page_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![prep[0], prep[1], 0xe7, mem_pos as u8, 0x00]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe7_sbc_zero_page_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xe7,
        mem_pos as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf7_sbc_zero_page_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = 112;

    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xf7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xf7_sbc_zero_page_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf7_sbc_zero_page_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xf7_sbc_zero_page_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xf7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf7_sbc_zero_page_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x10;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);

    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xf7,
        (mem_pos - 1) as u8,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xef_sbc_absolute() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xef,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xef_sbc_absolute_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xef,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xef_sbc_absolute_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xef,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xef_sbc_absolute_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        0xef,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xef_sbc_absolute_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = mem_pos.to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        0xef,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xff_sbc_absolute_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xff,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xff_sbc_absolute_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xff,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xff_sbc_absolute_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xff,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xff_sbc_absolute_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xff,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xff_sbc_absolute_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xff,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xfb_sbc_absolute_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xfb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xfb_sbc_absolute_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xfb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xfb_sbc_absolute_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xfb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xfb_sbc_absolute_y_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xfb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xfb_sbc_absolute_y_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_pos = 0x1000;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write(mem_pos, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    let mem_bytes = (mem_pos - 1).to_le_bytes();
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xfb,
        mem_bytes[0],
        mem_bytes[1],
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe3_sbc_indirect_x() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        cpu_test_helper::set_carry(),
        0xe3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos_indirect),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xe3_sbc_indirect_x_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos_indirect),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xe3_sbc_indirect_x_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos_indirect),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xe3_sbc_indirect_x_without_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_x_by_one(),
        0xe3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos_indirect),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xe3_sbc_indirect_x_with_carry_flag_overflow_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0001;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_x_by_one(),
        0xe3,
        mem_to_load - 1,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos_indirect),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf3_sbc_indirect_y() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 80;
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        cpu_test_helper::set_carry(),
        0xf3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos_indirect + 1),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
}

#[test]
fn test_0xf3_sbc_indirect_y_zero_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 10;
    let mem_value = accum_value - 1;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.mem_read(mem_pos_indirect + 1),mem_value);
    cpu_test_helper::assert_active_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}

#[test]
fn test_0xf3_sbc_indirect_y_negative_flag() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 10;
    let mem_value = 20;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos_indirect + 1),mem_value);
    cpu_test_helper::assert_active_negative_flag(&cpu);
    cpu_test_helper::assert_inactive_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
}

#[test]
fn test_0xf3_sbc_indirect_y_without_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = 80;
    let mem_value = (accum_value + 1 as u8).wrapping_neg();
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::increase_y_by_one(),
        0xf3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(
        cpu.register_a,
        accum_value.wrapping_sub(mem_value).wrapping_sub(1)
    );
    assert_eq!(cpu.mem_read(mem_pos_indirect + 1),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_inactive_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_active_negative_flag(&cpu);
}

#[test]
fn test_0xf3_sbc_indirect_y_with_carry_flag_overflow_fla() {
    let bus = Bus::new(test_rom(0x0600, None));

    let mut cpu = CPU::new(bus);
    let mem_to_load: u8 = 0x40;
    let mem_pos_indirect = 0x0010;
    let accum_value = (48 as u8).wrapping_neg();
    let mem_value = 112;
    cpu.mem_write_u16(mem_to_load as u16, mem_pos_indirect);
    cpu.mem_write(mem_pos_indirect + 1, mem_value - 1);

    let prep = cpu_test_helper::set_accumulator_to_value(accum_value);
    cpu.load_and_run(vec![
        prep[0],
        prep[1],
        cpu_test_helper::set_carry(),
        cpu_test_helper::increase_y_by_one(),
        0xf3,
        mem_to_load,
        0x00,
    ]);

    assert_eq!(cpu.register_a, accum_value.wrapping_sub(mem_value));
    assert_eq!(cpu.mem_read(mem_pos_indirect + 1),mem_value);
    cpu_test_helper::assert_active_overflow_flag(&cpu);
    cpu_test_helper::assert_active_carry_flag(&cpu);
    cpu_test_helper::assert_inactive_zero_flag(&cpu);
    cpu_test_helper::assert_inactive_negative_flag(&cpu);
}
