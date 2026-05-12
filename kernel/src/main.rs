#![no_main]
#![no_std]

mod allocator;

extern crate alloc; //created support but cannot use until allocator and its' error handler exists

use crate::allocator::bump::{ALLOCATOR, HEAP_SIZE, HEAP_START};
extern crate common;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_bi: &'static BootInfo, width: u64) -> ! {
    unsafe {
        let bi = BootInfo {
            framebuffer_ptr: 2147483648 as *mut u32,
            framebuffer_width: width,
            framebuffer_height: 1080,
            framebuffer_pitch: 1920,
        };

        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);

        use alloc::vec::Vec;
        let mut v = Vec::new();
        v.push(11);
        // bi;
        for x in 0..bi.framebuffer_width {
            for y in 0..bi.framebuffer_height {
                let pixel = bi
                    .framebuffer_ptr
                    .add((y * bi.framebuffer_pitch + x) as usize);

                *pixel = ((255 << 24) | ((x % 255) << 16) | ((y % 255) << 8) | 255) as u32; // ARGB
            }
        }
    }
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
