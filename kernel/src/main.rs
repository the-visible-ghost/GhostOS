#![no_std]
#![no_main]

extern crate common;

mod bootstrap;
mod memory;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(boot_info: &mut common::BootInfo) -> ! {
    let mut i: usize = 0;
    while i < 1920 * 1080 {
        boot_info.framebuffer.frame[i] = (255 << 24) | ((i as u32) << 16) | ((i as u32) << 8);
        i += 1;
    }

    boot_info.framebuffer.test1();

    // TODO: Before anything could be executed, take over paging
    // or else non-inline function calls will page fault immediately

    // The page tables will contain identity for the kernel
    // so that current execution wont give page fault.
    //
    // And the page tables will contain higher-half for the kernel
    // so that kernel could transition to the virtual space.
    //
    // also map framebuffer and boot_info (identity or custom)

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
