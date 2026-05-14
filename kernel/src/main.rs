#![no_main]
#![no_std]

mod allocator;
extern crate alloc;
use crate::allocator::bump::{ALLOCATOR, HEAP_SIZE};

extern crate common;

use alloc::vec::Vec;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "sysv64" fn kernel_main(boot_info: &'static mut common::BootInfo) -> ! {
    let fb = &mut boot_info.framebuffer;

    let heap_start =
        allocator::bump::find_heap_region(&boot_info.memory_map).expect("No usable heap memory");

    ALLOCATOR.lock().init(0x800000, HEAP_SIZE);
    let mut v = Vec::<u32>::new();
    // v.push(11);

    loop {
        fb.test1();
        for _ in 0..200_000_000 {}
        fb.test2();
        for _ in 0..200_000_000 {}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        *(0xdeadbeef as *mut u64) = 123;
    }
    loop {}
}
