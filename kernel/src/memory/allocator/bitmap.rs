use common::{
    gfx::buffer::Buffer,
    mmap::{MemoryMap, MemoryType},
};

const PAGE_SIZE: usize = 4096;

#[inline]
fn page_index(addr: usize) -> usize {
    addr / PAGE_SIZE
}

#[inline]
fn page_addr(index: usize) -> usize {
    index * PAGE_SIZE
}

pub struct BitmapAllocator {
    bitmap: *mut u8,
    bitmap_len: usize,
    memory_start: usize,
    memory_end: usize,
}

// Final alloc_pag and free_page functions
impl BitmapAllocator {
    pub unsafe fn alloc_page(&mut self) -> *mut u8 {
        let total_pages = (self.memory_end - self.memory_start) / PAGE_SIZE;

        unsafe {
            for i in 0..total_pages {
                if !self.test(i) {
                    self.set(i);
                    return (i * PAGE_SIZE) as *mut u8;
                }
            }
        }

        core::ptr::null_mut()
    }

    pub unsafe fn free_page(&mut self, addr: usize) {
        let idx = page_index(addr);
        unsafe { self.clear(idx) };
    }
}

// Bit Manipulation
impl BitmapAllocator {
    #[inline]
    unsafe fn set(&mut self, index: usize) {
        unsafe { *self.bitmap.add(index / 8) |= 1 << (index % 8) };
    }

    #[inline]
    unsafe fn clear(&mut self, index: usize) {
        unsafe { *self.bitmap.add(index / 8) &= !(1 << (index % 8)) };
    }

    #[inline]
    unsafe fn test(&self, index: usize) -> bool {
        (unsafe { *self.bitmap.add(index / 8) } & (1 << (index % 8))) != 0
    }
}

// General functions
impl BitmapAllocator {
    pub fn new(bitmap_ptr: *mut u8) -> Self {
        Self {
            bitmap: bitmap_ptr,
            bitmap_len: 0,
            memory_start: 0,
            memory_end: 0,
        }
    }

    pub unsafe fn init(&mut self, fb: &mut Buffer, mmap: &MemoryMap) {
        let mut max_addr = 0;

        for e in unsafe { mmap.entries() } {
            match MemoryType::from_u32(e.memorytype) {
                MemoryType::CONVENTIONAL => {
                    max_addr = max_addr.max(e.phys_start + e.page_count * PAGE_SIZE as u64)
                }
                _ => {}
            }
        }

        self.memory_start = 0;
        self.memory_end = max_addr as usize;

        self.bitmap_len = (max_addr as usize / PAGE_SIZE).div_ceil(8);

        unsafe { core::ptr::write_bytes(self.bitmap, 0xFF, self.bitmap_len) };

        for e in unsafe { mmap.entries() } {
            if e.phys_start == self.bitmap as u64 {
                continue;
            }
            match MemoryType::from_u32(e.memorytype) {
                MemoryType::CONVENTIONAL => unsafe {
                    self.free_range(e.phys_start as usize, e.page_count as usize * PAGE_SIZE)
                },
                _ => {}
            }
        }
    }

    unsafe fn free_range(&mut self, start: usize, size: usize) {
        let mut addr = start;
        let end = start + size;
        while addr < end {
            unsafe { self.free_page(addr) };
            addr += PAGE_SIZE;
        }
    }
}
