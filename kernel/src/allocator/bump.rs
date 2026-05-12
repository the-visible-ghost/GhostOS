pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

pub const HEAP_START: usize = 0x4444_4444_0000; //hardcoded heap area for testing
pub const HEAP_SIZE: usize = 1024 * 1024; // same as above

use core::alloc::{GlobalAlloc, Layout};

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
    pub const fn init(&mut self, heap_start: usize, heap_size: usize) {
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

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

#[global_allocator]
pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
