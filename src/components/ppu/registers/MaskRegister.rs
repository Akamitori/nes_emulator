#[allow(non_camel_case_types)]
pub struct MaskRegister {
    status: u8,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

impl MaskRegister {
    const GREYSCALE: u8 = 0b00000001;
    const SHOW_BACKGROUND_IN_LEFTMOST_8_PIXELS: u8 = 0b00000010;
    const SHOW_SPRITES_IN_LEFTMOST_8_PIXELS: u8 = 0b00000100;
    const SHOW_BACKGROUND: u8 = 0b00001000;
    const SHOW_SPRITES: u8 = 0b00010000;
    const EMPHASIZE_RED: u8 = 0b00100000;
    const EMPHASIZE_GREEN: u8 = 0b01000000;
    const EMPHASIZE_BLUE: u8 = 0b10000000;

    pub fn new() -> Self {
        MaskRegister {
            status: 0
        }
    }

    pub fn update(&mut self, status: u8) {
        self.status = status;
    }

    pub fn is_grey_scale(&self) -> bool {
        self.status & MaskRegister::GREYSCALE != 0
    }

    pub fn show_background_leftmost_8_pixels(&self) -> bool {
        self.status & MaskRegister::SHOW_BACKGROUND_IN_LEFTMOST_8_PIXELS != 0
    }

    pub fn show_sprites_leftmost_8_pixels(&self) -> bool {
        self.status & MaskRegister::SHOW_SPRITES_IN_LEFTMOST_8_PIXELS != 0
    }

    pub fn show_background(&self) -> bool {
        self.status & MaskRegister::SHOW_BACKGROUND != 0
    }

    pub fn show_sprites(&self) -> bool {
        self.status & MaskRegister::SHOW_SPRITES != 0
    }

    pub fn emphasize(&self) -> Vec<Color> {
        let mut result = vec![];

        if self.status & MaskRegister::EMPHASIZE_RED != 0 {
            result.push(Color::Red);
        }

        if self.status & MaskRegister::EMPHASIZE_GREEN != 0 {
            result.push(Color::Green);
        }

        if self.status & MaskRegister::EMPHASIZE_BLUE != 0 {
            result.push(Color::Blue);
        }

        result
    }
}