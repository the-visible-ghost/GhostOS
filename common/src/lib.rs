#![no_std]

use gfx::buffer::Buffer;

pub mod gfx;

#[repr(C)]
pub struct BootInfo<'a> {
    pub framebuffer: Buffer<'a>,
}
