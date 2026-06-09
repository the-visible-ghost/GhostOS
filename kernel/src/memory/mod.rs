pub mod allocator;
pub mod map;
pub mod paging;

#[inline(always)]
pub unsafe fn get_page_table_ptr() -> *mut u64 {
    let cr3: *mut u64;
    unsafe {
        core::arch::asm!(
            "mov {}, cr3",
            out(reg) cr3,
            options(nostack, preserves_flags)
        );
    }
    cr3
}

#[inline(always)]
pub unsafe fn is_5level_paging_enabled() -> bool {
    let cr4: u64;
    unsafe {
        core::arch::asm!(
            "mov {}, cr4",
            out(reg) cr4,
            options(nostack, preserves_flags)
        );
    }
    cr4 & (1 << 12) != 0
}
