#![no_std]

use gfx::buffer::Buffer;

use crate::{mmap::MemoryMap, phdrs::Headers};

pub mod gfx;
pub mod mmap;
pub mod phdrs;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo<'a> {
    pub framebuffer: Buffer<'a>,
    pub memory_map: MemoryMap,
    pub prog_headers: *mut Headers,
}
