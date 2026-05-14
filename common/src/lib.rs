#![no_std]

use gfx::buffer::Buffer;

use crate::mmap::MemoryMap;

pub mod gfx;
pub mod mmap;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo<'a> {
    pub framebuffer: Buffer<'a>,
    pub memory_map: MemoryMap,
}
