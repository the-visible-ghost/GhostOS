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

    #[inline]
    pub fn from_u32(color: u32) -> Self {
        Self {
            red: (color >> 16) as u8,
            green: (color >> 8) as u8,
            blue: color as u8,
            alpha: (color >> 24) as u8,
        }
    }

    #[inline]
    pub fn to_u32(self) -> u32 {
        (self.alpha as u32) << 24
            | (self.red as u32) << 16
            | (self.green as u32) << 8
            | (self.blue as u32)
    }

    #[inline]
    pub fn blend(self, target: Color) -> Self {
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
