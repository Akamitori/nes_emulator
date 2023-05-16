#[allow(non_camel_case_types)]
pub struct ControlRegister {
    status: u8,
}

impl ControlRegister {
    const NAMETABLE1: u8 = 0b00000001;
    const NAMETABLE2: u8 = 0b00000010;
    const VRAM_ADD_INCREMENT: u8 = 0b00000100;
    const SPRITE_PATTERN_ADDR: u8 = 0b00001000;
    const BACKROUND_PATTERN_ADDR: u8 = 0b00010000;
    const SPRITE_SIZE: u8 = 0b00100000;
    const MASTER_SLAVE_SELECT: u8 = 0b01000000;
    const GENERATE_NMI: u8 = 0b10000000;

    pub fn new() -> Self {
        ControlRegister {
            status: 0
        }
    }

    pub fn update(&mut self, data: u8) {
        self.status = data;
    }

    pub fn base_name_table_address(&self) -> u16 {
        let high_bit = self.status & ControlRegister::NAMETABLE2;
        let low_bit = self.status & ControlRegister::NAMETABLE1;

        let base_name_table_bits = high_bit | low_bit;
        return match base_name_table_bits {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Got a basename over 3 : {}", base_name_table_bits)
        };
    }


    pub fn vram_addr_increment(&self) -> u8 {
        if self.status & ControlRegister::VRAM_ADD_INCREMENT != 0 {
            1
        } else {
            32
        }
    }

    pub fn sprite_pattern_address_table(&self) -> u16 {
        if self.status & ControlRegister::SPRITE_PATTERN_ADDR == 0{
            0
        }
        else{
            0x1000
        }
    }
    
    pub fn background_pattern_table_address(&self) ->u16 {
        if self.status & ControlRegister::BACKROUND_PATTERN_ADDR == 0{
            0
        }
        else{
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if self.status & ControlRegister::SPRITE_SIZE == 1 {
            16
        }
        else{
            8
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        self.status & ControlRegister::MASTER_SLAVE_SELECT
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        self.status & ControlRegister::GENERATE_NMI != 0
    }
}

