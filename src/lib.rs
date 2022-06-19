mod opcodes;
use crate::opcodes::*;

#[cfg(test)]
mod tests;

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    memory: [u8; 0xFFFF],
    op_codes: OPCodes,
}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            memory: [0; 0xFFFF],
            op_codes: OPCodes::new(),
        }
    }

    const CARRY_FLAG: u8 = 0b0000_0001;
    const ZERO_FLAG: u8 = 0b0000_0010;
    const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
    const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
    const BREAK_COMMAND_FLAG: u8 = 0b0001_0000;
    const BREAK_COMMAND_FLAG_2: u8 = 0b0010_0000;
    const OVERFLOW_FLAG: u8 = 0b0100_0000;
    const NEGATIVE_FLAG: u8 = 0b1000_0000;

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let a = [self.mem_read(pos), self.mem_read(pos + 1)];
        let result = u16::from_le_bytes(a);
        return result;
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let le_bytes = data.to_le_bytes();
        self.mem_write(pos, le_bytes[0]);
        self.mem_write(pos + 1, le_bytes[1]);
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);

                let bytes = [
                    self.mem_read(ptr as u16),
                    self.mem_read(ptr.wrapping_add(1) as u16),
                ];
                u16::from_le_bytes(bytes)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let bytes = [
                    self.mem_read(base as u16),
                    self.mem_read(base.wrapping_add(1) as u16),
                ];

                let deref_base = u16::from_le_bytes(bytes);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flag(self.register_a);
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        println!();
        println!("the value we use isis {:#02x}", value);
        println!();
        self.set_register_a(self.register_a & value);
    }

    fn asl_accumulator(&mut self) {
        let mut data = self.register_a;
        if data >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        data <<= 1;
        self.set_register_a(data);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value <<= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);

            self.program_counter = jump_addr;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut mem_data = self.mem_read(addr);

        if (mem_data >> 6) & 0x1 == 1 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        if mem_data >> 7 == 1 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }

        mem_data &= self.register_a;

        if mem_data == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(value);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_and_negative_flag(self.register_x);
    }

    fn iny(&mut self) {
        if self.register_y == 0xFF {
            self.register_y = 0;
        } else {
            self.register_y += 1;
        }
        self.update_zero_and_negative_flag(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_a);
    }

    fn update_zero_and_negative_flag(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | CPU::ZERO_FLAG;
        } else {
            self.status = self.status & (!CPU::ZERO_FLAG);
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | CPU::NEGATIVE_FLAG;
        } else {
            self.status = self.status & (!CPU::NEGATIVE_FLAG);
        }
    }

    fn set_carry_flag(&mut self) {
        self.status |= CPU::CARRY_FLAG;
    }

    fn clear_carry_flag(&mut self) {
        self.status &= !CPU::CARRY_FLAG;
    }

    fn set_overflow_flag(&mut self) {
        self.status |= CPU::OVERFLOW_FLAG;
    }

    fn clear_overflow_flag(&mut self) {
        self.status &= !CPU::OVERFLOW_FLAG;
    }

    fn set_negative_flag(&mut self) {
        self.status |= CPU::NEGATIVE_FLAG;
    }

    fn clear_negative_flag(&mut self) {
        self.status &= !CPU::NEGATIVE_FLAG;
    }

    fn set_zero_flag(&mut self) {
        self.status |= CPU::ZERO_FLAG;
    }

    fn clear_zero_flag(&mut self) {
        self.status &= !CPU::ZERO_FLAG;
    }

    fn set_decimal_flag(&mut self) {
        self.status |= CPU::DECIMAL_MODE_FLAG;
    }

    fn clear_decimal_flag(&mut self) {
        self.status &= !CPU::DECIMAL_MODE_FLAG;
    }

    fn set_interrupt_flag(&mut self) {
        self.status |= CPU::INTERRUPT_DISABLE_FLAG;
    }

    fn clear_interrupt_flag(&mut self) {
        self.status &= !CPU::INTERRUPT_DISABLE_FLAG;
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let pc_temp = self.program_counter;

            println!("the op code is {:#02x}", code);

            //TO DO  add op code data to the original hashmap
            let op_code_data = self.op_codes.get(&code);

            match code {
                0x00 => return,
                0x0A => self.asl_accumulator(),
                0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&op_code_data.addressing_mode);
                }
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&op_code_data.addressing_mode);
                }

                0x90 => self.branch(self.status & CPU::CARRY_FLAG == 0),
                0xB0 => self.branch(self.status & CPU::CARRY_FLAG != 0),

                0xD0 => self.branch(self.status & CPU::ZERO_FLAG == 0),
                0xF0 => self.branch(self.status & CPU::ZERO_FLAG != 0),

                0x10 => self.branch(self.status & CPU::NEGATIVE_FLAG == 0),
                0x30 => self.branch(self.status & CPU::NEGATIVE_FLAG != 0),

                0x50 => self.branch(self.status & CPU::OVERFLOW_FLAG == 0),
                0x70 => self.branch(self.status & CPU::OVERFLOW_FLAG != 0),

                0x18 => self.clear_carry_flag(),
                0xD8 => self.clear_decimal_flag(),
                0x58 => self.clear_interrupt_flag(),
                0xB8 => self.clear_overflow_flag(),

                0xAA => self.tax(),
                0xA8 => self.tay(),
                0x24 | 0x2C => {
                    self.bit(&op_code_data.addressing_mode);
                }
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&op_code_data.addressing_mode);
                }
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&op_code_data.addressing_mode);
                }
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                _ => todo!(),
            }

            if pc_temp == self.program_counter {
                self.program_counter += (op_code_data.bytes - 1) as u16;
            }
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }
}
