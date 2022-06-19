#[cfg(test)]
pub mod test_helper {
    use std::mem::zeroed;

    use crate::*;

    const CARRY_FLAG: u8 = 0b0000_0001;
    const ZERO_FLAG: u8 = 0b0000_0010;
    const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
    const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
    const BREAK_COMMAND_FLAG: u8 = 0b0001_0000;
    const BREAK_COMMAND_FLAG_2: u8 = 0b0010_0000;
    const OVERFLOW_FLAG: u8 = 0b0100_0000;
    const NEGATIVE_FLAG: u8 = 0b1000_0000;

    pub fn assert_inactive_zero_carry_flags(cpu: CPU) {
        assert!(cpu.status & ZERO_FLAG == 0);
        assert!(cpu.status & CARRY_FLAG == 0);
    }

    pub fn assert_active_zero_flag(cpu: CPU) {
        assert!(cpu.status & ZERO_FLAG != 0);
    }

    pub fn assert_inactive_zero_flag(cpu: CPU) {
        assert!(cpu.status & ZERO_FLAG == 0);
    }

    pub fn assert_active_negative_flag(cpu: CPU) {
        assert!(cpu.status & NEGATIVE_FLAG != 0);
    }

    pub fn assert_active_overflow_flag(cpu: CPU) {
        assert!(cpu.status & OVERFLOW_FLAG != 0);
    }

    pub fn set_a_to_value(value_to_set: u8) -> Vec<u8> {
        let lda_direct_value = 0xa9;
        return vec![lda_direct_value, value_to_set];
    }
}
