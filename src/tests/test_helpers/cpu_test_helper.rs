﻿use std::collections::hash_map::Values;

use crate::components::cpu::CPU;
use crate::components::mem::Mem;

const CARRY_FLAG: u8 = 0b0000_0001;
const ZERO_FLAG: u8 = 0b0000_0010;
const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
const BREAK_COMMAND_FLAG_1: u8 = 0b0001_0000;
const BREAK_COMMAND_FLAG_2: u8 = 0b0010_0000;
const OVERFLOW_FLAG: u8 = 0b0100_0000;
const NEGATIVE_FLAG: u8 = 0b1000_0000;

pub fn assert_address_contains_value(cpu: &mut CPU, address: u16, value: u8) {
    assert_eq!(cpu.mem_read(address), value);
}

pub fn assert_inactive_zero_negative_flags(cpu: &CPU) {
    assert_inactive_zero_flag(cpu);
    assert_inactive_negative_flag(cpu);
}

pub fn assert_inactive_zero_carry_flags(cpu: &CPU) {
    assert_inactive_zero_flag(cpu);
    assert_inactive_carry_flag(cpu);
}

pub fn assert_inactive_zero_negative_carry_flag(cpu: &CPU) {
    assert_inactive_zero_flag(cpu);
    assert_inactive_negative_flag(cpu);
    assert_inactive_carry_flag(cpu);
}

pub fn assert_flags_unaffected(cpu: &CPU) {
    assert_eq!(cpu.status, CPU::STATUS_RESET);
}

pub fn assert_active_zero_flag(cpu: &CPU) {
    assert_ne!(cpu.status & ZERO_FLAG, 0);
}

pub fn assert_inactive_zero_flag(cpu: &CPU) {
    assert_eq!(cpu.status & ZERO_FLAG, 0);
}

pub fn assert_active_carry_flag(cpu: &CPU) {
    assert_ne!(cpu.status & CARRY_FLAG, 0);
}

pub fn assert_inactive_carry_flag(cpu: &CPU) {
    assert_eq!(cpu.status & CARRY_FLAG, 0);
}

pub fn assert_active_negative_flag(cpu: &CPU) {
    assert_ne!(cpu.status & NEGATIVE_FLAG, 0);
}

pub fn assert_inactive_negative_flag(cpu: &CPU) {
    assert_eq!(cpu.status & NEGATIVE_FLAG, 0);
}

pub fn assert_active_overflow_flag(cpu: &CPU) {
    assert_ne!(cpu.status & OVERFLOW_FLAG, 0);
}

pub fn assert_inactive_overflow_flag(cpu: &CPU) {
    assert_eq!(cpu.status & OVERFLOW_FLAG, 0);
}

pub fn assert_inactive_decimal_flag(cpu: &CPU) {
    assert_eq!(cpu.status & DECIMAL_MODE_FLAG, 0);
}

pub fn assert_active_decimal_flag(cpu: &CPU) {
    assert_ne!(cpu.status & DECIMAL_MODE_FLAG, 0);
}

pub fn assert_inactive_interrupt_flag(cpu: &CPU) {
    assert_eq!(cpu.status & INTERRUPT_DISABLE_FLAG, 0);
}

pub fn assert_active_interrupt_flag(cpu: &CPU) {
    assert_ne!(cpu.status & INTERRUPT_DISABLE_FLAG, 0);
}

pub fn assert_inactive_break_1_flag(cpu: &CPU) {
    assert_eq!(cpu.status & BREAK_COMMAND_FLAG_1, 0);
}

pub fn assert_active_break_1_flag(cpu: &CPU) {
    assert_ne!(cpu.status & BREAK_COMMAND_FLAG_1, 0);
}

pub fn assert_inactive_break_2_flag(cpu: &CPU) {
    assert_eq!(cpu.status & BREAK_COMMAND_FLAG_2, 0);
}

pub fn assert_active_break_2_flag(cpu: &CPU) {
    assert_ne!(cpu.status & BREAK_COMMAND_FLAG_2, 0);
}

pub fn set_accumulator_to_value(value_to_set: u8) -> [u8; 2] {
    let lda_direct_value = 0xa9;
    return [lda_direct_value, value_to_set];
}

pub fn store_accumulator_to_memory_address(address_to_store: u16) -> [u8; 3] {
    let sta_absolute = 0x8d;
    let memory_bytes = address_to_store.to_le_bytes();
    return [sta_absolute, memory_bytes[0], memory_bytes[1]];
}

pub fn set_register_x_to_value(value_to_set: u8) -> [u8; 2] {
    let ldx_direct_value = 0xa2;
    return [ldx_direct_value, value_to_set];
}

pub fn set_register_x_from_memory(address: u16) -> [u8; 3] {
    let ldx_absolute = 0xae;
    let address_bytes = address.to_le_bytes();
    return [ldx_absolute, address_bytes[0], address_bytes[1]];
}

pub fn set_register_y_to_value(value_to_set: u8) -> [u8; 2] {
    let ldy_direct_value = 0xa0;
    return [ldy_direct_value, value_to_set];
}

pub fn set_register_y_from_memory(address: u16) -> [u8; 3] {
    let ldy_absolute = 0xac;
    let address_bytes = address.to_le_bytes();
    return [ldy_absolute, address_bytes[0], address_bytes[1]];
}

pub fn store_register_x_to_zero_page(address: u8) -> [u8; 2] {
    return [0x86, address];
}

pub fn set_carry() -> u8 { return 0x38; }

pub fn clear_carry() -> u8 { return 0x18; }

pub fn add_with_carry_to_register_a(value_to_add: u8) -> [u8; 2] {
    return [0x69, value_to_add];
}

pub fn push_accumulator_to_stack() -> u8 {
    return 0x48;
}

pub fn store_register_to_address_zero_page(address_to_store: u8) -> [u8; 2] {
    let sta_zero_page = 0x85;
    return [sta_zero_page, address_to_store];
}

pub fn pull_stack_into_accumulator() -> u8 {
    return 0x68;
}

pub fn increase_x_by_one() -> u8 {
    return 0xe8;
}

pub fn increase_y_by_one() -> u8 {
    return 0xc8;
}

pub fn compare_x_to_value(value: u8) -> [u8; 2] {
    let cpx_immediate = 0xe0;
    return [cpx_immediate, value];
}

pub fn branch_not_equal(offset: u8) -> [u8; 2] {
    let bne = 0xd0;
    return [bne, offset];
}