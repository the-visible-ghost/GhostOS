#![no_main]
#![no_std]

extern crate common;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(boot_info: &'static mut common::BootInfo) -> ! {
    let fb = &mut boot_info.framebuffer;
    fb.test();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        *(0xdeadbeef as *mut u64) = 123;
    }
    loop {}
}
