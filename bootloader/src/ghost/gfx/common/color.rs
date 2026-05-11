use uefi::proto::console::gop::BltPixel;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_blt(pixel: BltPixel, alpha: u8) -> Self {
        Self {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
            alpha,
        }
    }

    pub fn blend_on(self, target: Color) -> Self {
        let alpha = self.alpha as u32;
        let inv_alpha = 255 - alpha;
        Self {
            red: ((self.red as u32 * alpha + target.red as u32 * inv_alpha) / 255) as u8,
            green: ((self.green as u32 * alpha + target.green as u32 * inv_alpha) / 255) as u8,
            blue: ((self.blue as u32 * alpha + target.blue as u32 * inv_alpha) / 255) as u8,
            alpha: 255,
        }
    }
}
