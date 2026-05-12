#![no_std]

use gfx::buffer::Buffer;

pub mod gfx;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo<'a> {
    pub framebuffer: Buffer<'a>,
}
