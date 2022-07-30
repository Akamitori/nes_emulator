use std::collections::HashMap;

use crate::AddressingMode;

#[derive(Copy,Clone)]
pub struct OpCode {
    pub op_code: u8,
    pub command_name: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode,
}

impl OpCode {
    pub fn new(
        op_code: u8,
        command_name: &'static str,
        bytes: u8,
        cycles: u8,
        addressing_mode: AddressingMode,
    ) -> Self {
        OpCode {
            op_code,
            command_name: command_name,
            bytes,
            cycles,
            addressing_mode,
        }
    }
}

pub struct OPCodes{
    codes:HashMap<u8,OpCode>
}

impl OPCodes{
    pub fn new()-> Self{
        let mut code_table=vec![
        
        OpCode::new(0xAA,"TAX",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0xA8,"TAY",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x29,"AND",2,2,AddressingMode::Immediate),
        OpCode::new(0x25,"AND",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x35,"AND",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x2D,"AND",3,4,AddressingMode::Absolute),
        OpCode::new(0x3D,"AND",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),
        OpCode::new(0x39,"AND",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),
        OpCode::new(0x21,"AND",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x31,"AND",2,5/*+1 if page crossed*/,AddressingMode::Indirect_Y),
        
        OpCode::new(0x0A,"ASL",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x06,"ASL",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x16,"ASL",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x0E,"ASL",3,6,AddressingMode::Absolute),
        OpCode::new(0x1E,"ASL",3,7,AddressingMode::Absolute_X),

        OpCode::new(0x90,"BCC",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),
        
        OpCode::new(0xB0,"BCS",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),
        
        OpCode::new(0xF0,"BEQ",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),

        OpCode::new(0x24,"BIT",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x2C,"BIT",3,4,AddressingMode::Absolute),

        OpCode::new(0x30,"BMI",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),
        
        OpCode::new(0xD0,"BNE",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),
        
        OpCode::new(0x10,"BPL",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),

        OpCode::new(0x00,"BRK",1,7,AddressingMode::NoneAddressing),

        OpCode::new(0x50,"BVC",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),
       
        OpCode::new(0x70,"BVS",2,2/*+1 if branch succeeds OR +2 if to a new page*/,AddressingMode::NoneAddressing),

        OpCode::new(0x18,"CLC",1,2,AddressingMode::NoneAddressing),
        
        OpCode::new(0xD8,"CLD",1,2,AddressingMode::NoneAddressing),
        
        OpCode::new(0x58,"CLI",1,2,AddressingMode::NoneAddressing),
        
        OpCode::new(0xB8,"CLV",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0xC9,"CMP",2,2,AddressingMode::Immediate),
        OpCode::new(0xC5,"CMP",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xD5,"CMP",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xCD,"CMP",3,4,AddressingMode::Absolute),
        OpCode::new(0xDD,"CMP",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),
        OpCode::new(0xD9,"CMP",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),
        OpCode::new(0xC1,"CMP",2,6,AddressingMode::Indirect_X),
        OpCode::new(0xD1,"CMP",2,5/*+1 if page crossed*/,AddressingMode::Indirect_Y),

        OpCode::new(0xE0,"CPX",2,2,AddressingMode::Immediate),
        OpCode::new(0xE4,"CPX",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xEC,"CPX",3,4,AddressingMode::Absolute),

        OpCode::new(0xC0,"CPY",2,2,AddressingMode::Immediate),
        OpCode::new(0xC4,"CPY",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xCC,"CPY",3,4,AddressingMode::Absolute),

        OpCode::new(0xC6,"DEC",2,5,AddressingMode::ZeroPage),
        OpCode::new(0xD6,"DEC",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0xCE,"DEC",3,6,AddressingMode::Absolute),
        OpCode::new(0xDE,"DEC",3,7,AddressingMode::Absolute_X),

        OpCode::new(0xCA,"DEX",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x88,"DEY",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x49,"EOR",2,2,AddressingMode::Immediate),
        OpCode::new(0x45,"EOR",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x55,"EOR",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x4D,"EOR",3,4,AddressingMode::Absolute),
        OpCode::new(0x5D,"EOR",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),
        OpCode::new(0x59,"EOR",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),
        OpCode::new(0x41,"EOR",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x51,"EOR",2,5/*+1 if page crossed*/,AddressingMode::Indirect_Y),

        OpCode::new(0xE6,"INC",2,5,AddressingMode::ZeroPage),
        OpCode::new(0xF6,"INC",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0xEE,"INC",3,6,AddressingMode::Absolute),
        OpCode::new(0xFE,"INC",3,7,AddressingMode::Absolute_X),

        OpCode::new(0xE8,"INX",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0xC8,"INY",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x4C,"JMP",3,3,AddressingMode::Absolute),
        OpCode::new(0x6C,"JMP",3,5,AddressingMode::NoneAddressing),

        OpCode::new(0x20,"JSR",3,6,AddressingMode::Absolute),

        OpCode::new(0xA9,"LDA",2,2,AddressingMode::Immediate),
        OpCode::new(0xA5,"LDA",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xB5,"LDA",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xAD,"LDA",3,4,AddressingMode::Absolute),
        OpCode::new(0xBD,"LDA",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),
        OpCode::new(0xB9,"LDA",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),
        OpCode::new(0xA1,"LDA",2,6,AddressingMode::Indirect_X),
        OpCode::new(0xB1,"LDA",2,5/*+1 if page crossed*/,AddressingMode::Indirect_Y),

        OpCode::new(0xA2,"LDX",2,2,AddressingMode::Immediate),
        OpCode::new(0xA6,"LDX",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xB6,"LDX",2,4,AddressingMode::ZeroPage_Y),
        OpCode::new(0xAE,"LDX",3,4,AddressingMode::Absolute),
        OpCode::new(0xBE,"LDX",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),

        OpCode::new(0xA0,"LDY",2,2,AddressingMode::Immediate),
        OpCode::new(0xA4,"LDY",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xB4,"LDY",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xAC,"LDY",3,4,AddressingMode::Absolute),
        OpCode::new(0xBC,"LDY",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),

        OpCode::new(0x4A,"LSR",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x46,"LSR",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x56,"LSR",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x4E,"LSR",3,6,AddressingMode::Absolute),
        OpCode::new(0x5E,"LSR",3,7,AddressingMode::Absolute_X),

        OpCode::new(0xEA,"NOP",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x09,"ORA",2,2,AddressingMode::Immediate),
        OpCode::new(0x05,"ORA",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x15,"ORA",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x0D,"ORA",3,4,AddressingMode::Absolute),
        OpCode::new(0x1D,"ORA",3,4/*+1 if page crossed*/,AddressingMode::Absolute_X),
        OpCode::new(0x19,"ORA",3,4/*+1 if page crossed*/,AddressingMode::Absolute_Y),
        OpCode::new(0x01,"ORA",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x11,"ORA",2,5/*+1 if page crossed*/,AddressingMode::Indirect_Y),

        OpCode::new(0x48,"PHA",1,3,AddressingMode::NoneAddressing),

        OpCode::new(0x08,"PHP",1,3,AddressingMode::NoneAddressing),

        OpCode::new(0x68,"PLA",1,4,AddressingMode::NoneAddressing),

        OpCode::new(0x28,"PLP",1,4,AddressingMode::NoneAddressing),

        OpCode::new(0x60,"RTS ",1,6,AddressingMode::NoneAddressing),

        OpCode::new(0x2A,"ROL",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x26,"ROL",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x36,"ROL",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x2E,"ROL",3,6,AddressingMode::Absolute),
        OpCode::new(0x3E,"ROL",3,7,AddressingMode::Absolute_X),

        OpCode::new(0x6A,"ROR ",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x66,"ROR ",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x76,"ROR ",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x6E,"ROR ",3,6,AddressingMode::Absolute),
        OpCode::new(0x7E,"ROR ",3,7,AddressingMode::Absolute_X),

        OpCode::new(0x38,"SEC ",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0xF8,"SED ",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x78,"SEI ",1,2,AddressingMode::NoneAddressing),

        OpCode::new(0x85,"STA",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x95,"STA",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x8D,"STA",3,4,AddressingMode::Absolute),
        OpCode::new(0x9D,"STA",3,5,AddressingMode::Absolute_X),
        OpCode::new(0x99,"STA",3,5,AddressingMode::Absolute_Y),
        OpCode::new(0x81,"STA",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x91,"STA",2,6,AddressingMode::Indirect_Y),
        ];
        
        code_table.sort_by(|a,b| a.op_code.cmp(&b.op_code) );

        let mut codes=HashMap::new();

        for c in code_table{
            if codes.contains_key(&c.op_code) {
                panic!("Duplicate key {:#02x}",c.op_code);
            }
            codes.insert(c.op_code, c);
        }
        
        OPCodes{
            codes
        }
    }

    pub fn get(&self,op_code: &u8)-> OpCode{
        return *self.codes.get(op_code).unwrap();
    }
}