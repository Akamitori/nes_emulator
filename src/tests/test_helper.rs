use crate::CPU;

const CARRY_FLAG: u8 = 0b0000_0001;
const ZERO_FLAG: u8 = 0b0000_0010;
const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
const BREAK_COMMAND_FLAG_1: u8 = 0b0001_0000;
const BREAK_COMMAND_FLAG_2: u8 = 0b0010_0000;
const OVERFLOW_FLAG: u8 = 0b0100_0000;
const NEGATIVE_FLAG: u8 = 0b1000_0000;

pub fn assert_address_contains_value(cpu: &CPU, address: u16, value: u8) {
    assert_eq!(cpu.mem_read(address), value);
}

pub fn assert_inactive_zero_negative_flags(cpu: &CPU) {
    assert!(cpu.status & ZERO_FLAG == 0);
    assert!(cpu.status & NEGATIVE_FLAG == 0);
}

pub fn assert_inactive_zero_carry_flags(cpu: &CPU) {
    assert!(cpu.status & ZERO_FLAG == 0);
    assert!(cpu.status & CARRY_FLAG == 0);
}

pub fn assert_inactive_zero_negative_carry_flag(cpu: &CPU) {
    assert!(cpu.status & ZERO_FLAG == 0);
    assert!(cpu.status & NEGATIVE_FLAG == 0);
    assert!(cpu.status & CARRY_FLAG == 0);
}

pub fn assert_active_zero_flag(cpu: &CPU) {
    assert!(cpu.status & ZERO_FLAG != 0);
}

pub fn assert_inactive_zero_flag(cpu: &CPU) {
    assert!(cpu.status & ZERO_FLAG == 0);
}

pub fn assert_active_carry_flag(cpu: &CPU) {
    assert!(cpu.status & CARRY_FLAG != 0);
}

pub fn assert_inactive_carry_flag(cpu: &CPU) {
    assert!(cpu.status & CARRY_FLAG == 0);
}

pub fn assert_active_negative_flag(cpu: &CPU) {
    assert!(cpu.status & NEGATIVE_FLAG != 0);
}

pub fn assert_inactive_negative_flag(cpu: &CPU) {
    assert!(cpu.status & NEGATIVE_FLAG == 0);
}

pub fn assert_active_overflow_flag(cpu: &CPU) {
    assert!(cpu.status & OVERFLOW_FLAG != 0);
}

pub fn assert_inactive_overflow_flag(cpu: &CPU) {
    assert!(cpu.status & OVERFLOW_FLAG == 0);
}

pub fn assert_inactive_decimal_flag(cpu: &CPU) {
    assert!(cpu.status & DECIMAL_MODE_FLAG == 0);
}

pub fn assert_active_decimal_flag(cpu: &CPU) {
    assert!(cpu.status & DECIMAL_MODE_FLAG != 0);
}

pub fn assert_inactive_interrupt_flag(cpu: &CPU) {
    assert!(cpu.status & INTERRUPT_DISABLE_FLAG == 0);
}

pub fn assert_active_interrupt_flag(cpu: &CPU) {
    assert!(cpu.status & INTERRUPT_DISABLE_FLAG != 0);
}

pub fn assert_inactive_break_1_flag(cpu: &CPU) {
    assert!(cpu.status & BREAK_COMMAND_FLAG_1 == 0);
}

pub fn assert_active_break_1_flag(cpu: &CPU) {
    assert!(cpu.status & BREAK_COMMAND_FLAG_1 != 0);
}

pub fn assert_inactive_break_2_flag(cpu: &CPU) {
    assert!(cpu.status & BREAK_COMMAND_FLAG_2 == 0);
}

pub fn assert_active_break_2_flag(cpu: &CPU) {
    assert!(cpu.status & BREAK_COMMAND_FLAG_2 != 0);
}

pub fn set_register_a_to_value(value_to_set: u8) -> [u8; 2] {
    let lda_direct_value = 0xa9;
    return [lda_direct_value, value_to_set];
}

pub fn increase_x_by_one() -> u8 {
    return 0xe8;
}

pub fn increase_y_by_one() -> u8 {
    return 0xc8;
}
