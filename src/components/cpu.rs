﻿use crate::components::bus::Bus;
use crate::components::mem::Mem;
use crate::components::opcodes::OPCodes;

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    op_codes: OPCodes,
    pub  stack_pointer: u8,
    pub bus: Bus
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

// 0x0600 for snake game
const INITIAL_MEMORY_LOAD_ADDRESS: u16 = 0x0600;

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }
    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
    }
}

impl CPU {
    pub fn new(bus : Bus) -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            op_codes: OPCodes::new(),
            stack_pointer: CPU::STACK_RESET,
            bus: bus
        }
    }

    const CARRY_FLAG: u8 = 0b0000_0001;
    const ZERO_FLAG: u8 = 0b0000_0010;
    const INTERRUPT_DISABLE_FLAG: u8 = 0b0000_0100;
    const DECIMAL_MODE_FLAG: u8 = 0b0000_1000;
    const BREAK_COMMAND_FLAG_1: u8 = 0b0001_0000;
    const BREAK_COMMAND_FLAG_2: u8 = 0b0010_0000;
    const OVERFLOW_FLAG: u8 = 0b0100_0000;
    const NEGATIVE_FLAG: u8 = 0b1000_0000;

    const STACK_BOTTOM: u16 = 0x0100;
    pub  const STACK_RESET: u8 = 0xFD;
    pub  const STATUS_RESET :u8=CPU::INTERRUPT_DISABLE_FLAG | CPU::BREAK_COMMAND_FLAG_2;

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

    fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.update_zero_and_negative_flag(self.register_x);
    }

    fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.update_zero_and_negative_flag(self.register_y);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16){
            self.mem_write(INITIAL_MEMORY_LOAD_ADDRESS+i,program[i as usize]);
        } 
        self.mem_write_u16(0xFFFC, INITIAL_MEMORY_LOAD_ADDRESS);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn add_to_register_a(&mut self, value: u8) {
        let accum_value = self.register_a;
        let carry = self.get_carry_flag();
        let sum = value as u16 + accum_value as u16 + carry as u16;

        let set_carry = sum > 0xff;
        if set_carry {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let result = sum as u8;

        let bit_7_mask = 0x80;

        if ((value ^ result) & (accum_value ^ result) & bit_7_mask) != 0 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }

        self.set_register_a(result);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

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

    fn bcc(&mut self) {
        self.branch(self.get_carry_flag() == 0);
    }

    fn bcs(&mut self) {
        self.branch(self.get_carry_flag() != 0);
    }

    fn beq(&mut self) {
        self.branch(self.status & CPU::ZERO_FLAG != 0);
    }

    fn bmi(&mut self) {
        self.branch(self.status & CPU::NEGATIVE_FLAG != 0);
    }

    fn bne(&mut self) {
        self.branch(self.status & CPU::ZERO_FLAG == 0);
    }

    fn bpl(&mut self) {
        self.branch(self.status & CPU::NEGATIVE_FLAG == 0);
    }

    fn bvc(&mut self) {
        self.branch(self.status & CPU::OVERFLOW_FLAG == 0);
    }

    fn bvs(&mut self) {
        self.branch(self.status & CPU::OVERFLOW_FLAG != 0);
    }

    fn clc(&mut self) {
        self.clear_carry_flag();
    }

    fn cld(&mut self) {
        self.clear_decimal_flag();
    }

    fn cli(&mut self) {
        self.clear_interrupt_flag();
    }

    fn clv(&mut self) {
        self.clear_overflow_flag();
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

    fn jmp(&mut self, mode: &AddressingMode) {
        let address_to_jump = self.get_operand_address(mode);
        self.program_counter = address_to_jump;
    }

    fn jmp_indirect(&mut self) {
        let address_to_read = self.mem_read_u16(self.program_counter);
        let on_page_boundary = address_to_read & 0x00FF == 0x00FF;

        let address_to_jump = if on_page_boundary {
            let lsb = self.mem_read(address_to_read);
            let msb = self.mem_read(address_to_read & 0xFF00);

            let a = [lsb, msb];
            u16::from_le_bytes(a)
        } else {
            self.mem_read_u16(address_to_read)
        };

        self.program_counter = address_to_jump;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let address_to_jump = self.get_operand_address(mode);
        self.stack_push_u16(self.program_counter + 2 - 1);
        self.program_counter = address_to_jump;
    }

    fn rts(&mut self) {
        let address_to_return = self.stack_pop_u16() + 1;
        self.program_counter = address_to_return;
    }

    fn rti(&mut self) {
        let mut status = self.stack_pop();
        status &= !CPU::BREAK_COMMAND_FLAG_1;
        status &= !CPU::BREAK_COMMAND_FLAG_2;
        self.status = status;
        let program_counter = self.stack_pop_u16();
        self.program_counter = program_counter;
    }

    fn rol_accumulator(&mut self) {
        let mut data = self.register_a;
        let bit_0 = self.get_carry_flag();
        if data >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        data <<= 1;
        data |= bit_0;
        self.set_register_a(data);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let bit_0 = self.get_carry_flag();
        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value <<= 1;
        value |= bit_0;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn ror_accumulator(&mut self) {
        let mut data = self.register_a;
        let bit_7 = self.get_carry_flag() << 7;
        if data & 1 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        data >>= 1;
        data |= bit_7;
        self.set_register_a(data);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let bit_7 = self.get_carry_flag() << 7;
        if value & 1 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value >>= 1;
        value |= bit_7;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn sec(&mut self) {
        self.set_carry_flag();
    }

    fn sed(&mut self) {
        self.set_decimal_flag();
    }

    fn sei(&mut self) {
        self.set_interrupt_flag();
    }

    fn stack_push(&mut self, value: u8) {
        self.mem_write(CPU::STACK_BOTTOM + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let data = self.mem_read(CPU::STACK_BOTTOM + self.stack_pointer as u16);
        return data;
    }

    fn stack_push_u16(&mut self, data: u16) {
        let bytes = data.to_le_bytes();
        self.stack_push(bytes[0]);
        self.stack_push(bytes[1]);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let hi = self.stack_pop();
        let lo = self.stack_pop();
        let bytes = [lo, hi];
        u16::from_le_bytes(bytes)
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

    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_a);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_x);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare(mode, self.register_y);
    }

    fn compare(&mut self, mode: &AddressingMode, compare_with: u8) {
        let addr = self.get_operand_address(mode);
        let mem_data = self.mem_read(addr);

        if compare_with >= mem_data {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_zero_and_negative_flag(compare_with.wrapping_sub(mem_data));
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_sub(1);
        self.mem_write(addr, data);
        self.update_zero_and_negative_flag(data);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        data = data.wrapping_add(1);
        self.mem_write(addr, data);
        self.update_zero_and_negative_flag(data);
    }

    fn dex(&mut self) {
        self.set_register_x(self.register_x.wrapping_sub(1));
    }

    fn dey(&mut self) {
        self.set_register_y(self.register_y.wrapping_sub(1));
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(value);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_x(value);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_y(value);
    }

    fn lsr_accumulator(&mut self) {
        let mut data = self.register_a;
        if data & 1 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        data >>= 1;
        self.set_register_a(data);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        if value & 1 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value >>= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flag(value);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(self.register_a | value);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn php(&mut self) {
        let mut status = self.status;
        status |= CPU::BREAK_COMMAND_FLAG_1 | CPU::BREAK_COMMAND_FLAG_2;
        self.stack_push(status);
    }

    fn pla(&mut self) {
        let value = self.stack_pop();
        self.set_register_a(value);
    }

    fn plp(&mut self) {
        let mut status = self.stack_pop();
        status &= !CPU::BREAK_COMMAND_FLAG_1;
        status &= !CPU::BREAK_COMMAND_FLAG_2;
        self.status = status;
    }

    fn tax(&mut self) {
        self.set_register_x(self.register_a);
    }

    fn tay(&mut self) {
        self.set_register_y(self.register_a);
    }

    fn inx(&mut self) {
        self.set_register_x(self.register_x.wrapping_add(1));
    }

    fn iny(&mut self) {
        self.set_register_y(self.register_y.wrapping_add(1));
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(self.register_a ^ value);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_y);
    }

    fn tsx(&mut self) {
        let value = self.stack_pointer;
        self.set_register_x(value);
    }

    fn txa(&mut self) {
        let value = self.register_x;
        self.set_register_a(value);
    }

    fn txs(&mut self) {
        let value = self.register_x;
        self.stack_pointer = value;
    }

    fn tya(&mut self) {
        let value = self.register_y;
        self.set_register_a(value);
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

    fn get_carry_flag(&self) -> u8 {
        self.status & CPU::CARRY_FLAG
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
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self,mut callback:F) where F: FnMut(&mut CPU) {
        loop {
            callback(self);
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let pc_temp = self.program_counter;

            // println!("the op code is {:#02x}", code);

            //TO DO  add op code data to the original hashmap
            let op_code_data = self.op_codes.get(&code);

            match code {
                0x00 => return,

                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&op_code_data.addressing_mode)
                }

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&op_code_data.addressing_mode);
                }

                0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&op_code_data.addressing_mode);
                }

                0x0A => self.asl_accumulator(),

                0x90 => self.bcc(),

                0xB0 => self.bcs(),

                0xF0 => self.beq(),

                0x24 | 0x2C => {
                    self.bit(&op_code_data.addressing_mode);
                }

                0x30 => self.bmi(),

                0xD0 => self.bne(),

                0x10 => self.bpl(),

                0x50 => self.bvc(),

                0x70 => self.bvs(),

                0x18 => self.clc(),

                0xD8 => self.cld(),

                0x58 => self.cli(),

                0xB8 => self.clv(),

                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&op_code_data.addressing_mode);
                }

                0xE0 | 0xE4 | 0xEC => {
                    self.cpx(&op_code_data.addressing_mode);
                }

                0xC0 | 0xC4 | 0xCC => {
                    self.cpy(&op_code_data.addressing_mode);
                }

                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.dec(&op_code_data.addressing_mode);
                }

                0xCA => {
                    self.dex();
                }

                0x88 => {
                    self.dey();
                }

                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&op_code_data.addressing_mode);
                }

                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.inc(&op_code_data.addressing_mode);
                }

                0xE8 => self.inx(),

                0xC8 => self.iny(),

                0x4C => self.jmp(&op_code_data.addressing_mode),

                0x6C => self.jmp_indirect(),

                0x20 => self.jsr(&op_code_data.addressing_mode),

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&op_code_data.addressing_mode);
                }

                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&op_code_data.addressing_mode);
                }

                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&op_code_data.addressing_mode);
                }

                0x4A => {
                    self.lsr_accumulator();
                }

                0x46 | 0x56 | 0x4E | 0x5E => {
                    self.lsr(&op_code_data.addressing_mode);
                }

                // NOP
                0xEA => {}

                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.ora(&op_code_data.addressing_mode);
                }

                0x48 => {
                    self.pha();
                }

                0x08 => {
                    self.php();
                }

                0x68 => {
                    self.pla();
                }

                0x28 => {
                    self.plp();
                }

                0x2A => self.rol_accumulator(),

                0x26 | 0x36 | 0x2E | 0x3E => {
                    self.rol(&op_code_data.addressing_mode);
                }

                0x6A => self.ror_accumulator(),

                0x66 | 0x76 | 0x6E | 0x7E => {
                    self.ror(&op_code_data.addressing_mode);
                }

                0x40 => self.rti(),

                0x60 => self.rts(),

                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(&op_code_data.addressing_mode);
                }

                0x38 => self.sec(),

                0xF8 => self.sed(),

                0x78 => self.sei(),

                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&op_code_data.addressing_mode);
                }

                0x86 | 0x96 | 0x8E => {
                    self.stx(&op_code_data.addressing_mode);
                }

                0x84 | 0x94 | 0x8C => {
                    self.sty(&op_code_data.addressing_mode);
                }

                0xAA => self.tax(),
                0xA8 => self.tay(),

                0xBA => self.tsx(),

                0x8A => self.txa(),

                0x9A => self.txs(),

                0x98 => self.tya(),

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
        self.stack_pointer = CPU::STACK_RESET;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }
}