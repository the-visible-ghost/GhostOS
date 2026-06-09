pub mod pt_arena;
pub mod pt_map;

use self::pt_arena::PtArena;

#[unsafe(no_mangle)]
pub extern "sysv64" fn bootstrap(boot_info: &mut common::BootInfo, pt_arena: *mut PtArena) {
    // NOTE: This function is called with firmware's page tables mappings [Mostly IDENTITY]

    let arena = unsafe { &mut *pt_arena };

    let pml4 = (unsafe { &mut *pt_arena }).allocate();
    pml4.clear_all();

    // Map kernel to higher half
    let headers = unsafe { &mut *boot_info.prog_headers };
    let mut index = 0;
    while index < headers.len {
        let header = unsafe { &mut *headers.ptr.add(index as usize) };

        pt_map::linear(
            pml4,
            arena,
            header.virt_addr as *mut u8,
            header.phys_addr as *mut u8,
            header.mem_size as usize >> 12,
            header.flags,
        );

        index += 1;
    }

    // Map GOP Framebuffer
    pt_map::linear(
        pml4,
        arena,
        boot_info.framebuffer.frame.as_mut_ptr() as *mut u8,
        boot_info.framebuffer.frame.as_mut_ptr() as *mut u8,
        (boot_info.framebuffer.width * boot_info.framebuffer.height) << 10,
        0b110,
    );
}
