extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};

pub struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        match x >= self.width || y >= self.height {
            true => None,
            false => self.pixels.get_mut(y * self.width + x),
        }
    }

    pub fn blit(&self, gop: &mut GraphicsOutput) -> Result<(), uefi::Error> {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels.resize(width * height, BltPixel::new(0, 0, 0));
    }
}
