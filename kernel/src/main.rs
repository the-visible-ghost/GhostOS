#![no_main]
#![no_std]

extern crate common;

use common::gfx;
use core::panic::PanicInfo;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer_ptr: *mut [u32],
    pub framebuffer_width: u64,
    pub framebuffer_height: u64,
    pub framebuffer_pitch: u64, // pixels per row
}

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(bi: &'static BootInfo) -> ! {
    let fb = gfx::buffer::Buffer::new(
        (
            bi.framebuffer_width as usize,
            bi.framebuffer_height as usize,
        ),
        bi.framebuffer_pitch as usize,
        unsafe { &mut *bi.framebuffer_ptr },
    );
    fb.text();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        *(0xdeadbeef as *mut u64) = 123;
    }
    loop {}
}
