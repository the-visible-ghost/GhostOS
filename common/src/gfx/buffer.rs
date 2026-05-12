use super::Position;
use super::color::Color;

use core::fmt;

pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

pub struct Buffer<'a> {
    pub width: usize,
    pub height: usize,
    pitch: usize,
    pub frame: &'a mut [u32],
}

impl Resolution {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl<'a> Buffer<'a> {
    #[inline]
    pub fn new(res: Resolution, pitch: usize, frame: &'a mut [u32]) -> Self {
        frame[0] = 0xFF0000FF;
        Self {
            width: res.width,
            height: res.height,
            pitch,
            frame,
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, pos: Position, color: Color) {
        if self.check_position(pos.x, pos.y) {
            self.set_pixel_unchecked(pos, color);
        }
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if !self.check_position(x, y) {
            return None;
        }
        Some(self.get_pixel_unchecked(x, y))
    }

    #[inline]
    fn check_position(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            return true;
        }
        false
    }

    #[inline]
    pub fn blend_pixel(&mut self, pos: Position, color: Color) {
        if self.check_position(pos.x, pos.y) {
            self.blend_pixel_unchecked(pos, color);
        }
    }

    #[inline]
    pub fn resize(&mut self, x: usize, y: usize) {
        self.width = x;
        self.height = y;
    }

    pub fn blackout(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(Position::new(x, y), Color::new(0, 0, 0, 0));
            }
        }
    }

    #[inline]
    pub fn test(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(
                    Position::new(x, y),
                    Color::new((x % 255) as u8, (y % 255) as u8, 255, 255),
                );
            }
        }
    }

    #[inline]
    fn get_pixel_unchecked(&self, x: usize, y: usize) -> Color {
        Color::from_u32(self.frame[y * self.pitch + x])
    }

    #[inline]
    fn set_pixel_unchecked(&mut self, pos: Position, color: Color) {
        self.frame[pos.y * self.pitch + pos.x] = color.to_u32();
    }

    #[inline]
    fn blend_pixel_unchecked(&mut self, pos: Position, color: Color) {
        let resultant = color.blend(self.get_pixel_unchecked(pos.x, pos.y));
        self.set_pixel_unchecked(pos, resultant);
    }
}

impl fmt::Debug for Buffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pitch", &self.pitch)
            .field("frame", &self.frame.as_ptr())
            .finish()
    }
}
