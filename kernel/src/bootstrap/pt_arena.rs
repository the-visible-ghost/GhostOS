use crate::memory::paging::PageTable;

#[repr(C)]
pub struct PtArena {
    ptr: *mut PageTable,
    len: u64,
}

impl PtArena {
    #[inline(always)]
    pub fn allocate(&mut self) -> &mut PageTable {
        let pt = unsafe { &mut *(self.ptr.add(self.len as usize)) };
        self.len += 1;
        pt
    }
}
