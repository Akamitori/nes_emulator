use crate::components::cartridge::Rom;
use crate::components::mem::Mem;
use crate::components::ppu::NesPPU;

pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: NesPPU,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            //mirror if needed
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2008;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const ROM_SPACE_START: u16 = 0x8000;
const ROM_SPACE_END: u16 = 0xFFFF;

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }

            //0x2002 => self.ppu.read_status(),

            0x2007 => self.ppu.read_data(),

            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
            }
            ROM_SPACE_START..=ROM_SPACE_END => {
                self.read_prg_rom(addr)
            }
            _ => {
                // println!("Ignoring mem access at {}", addr);
                panic!("not supported for now");
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }

            0x2000 => {
                self.ppu.write_to_ctrl(data);
            }

            0x2001 => {
                self.ppu.write_to_mask_register(data);
            }

            0x2005 => {
                self.ppu.write_to_scroll_register(data);
            }

            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }

            0x2007 => {
                self.ppu.write_to_data(data);
            }

            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }
            ROM_SPACE_START..=ROM_SPACE_END => {
                panic!("Attempt to write to Cartridge ROM space")
            }
            _ => {
                // println!("Ignoring mem write-access at {}", addr);
                panic!("not supported for now");
            }
        }
    }
}