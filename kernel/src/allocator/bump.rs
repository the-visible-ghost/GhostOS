pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

pub const HEAP_SIZE: usize = 1024 * 1024; // same as above

use core::alloc::{GlobalAlloc, Layout};

use common::mmap::{MemoryMap, MemoryType};
use spin::Mutex;

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

impl BumpAllocator {
    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }

    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
        }
    }
}

pub fn find_heap_region(mmap: &MemoryMap) -> Option<usize> {
    let entries = unsafe { core::slice::from_raw_parts(mmap.entries, mmap.num_entries) };

    for entry in entries {
        let typ = MemoryType::from_u32(entry.memorytype);

        if let Some(MemoryType::CONVENTIONAL) = typ {
            let region_size = entry.page_count as usize * 4096;

            if region_size <= HEAP_SIZE {
                return Some(entry.phys_start as usize);
            }
        }
    }
    None
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

#[global_allocator]
pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
