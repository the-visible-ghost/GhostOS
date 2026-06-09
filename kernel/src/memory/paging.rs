#[derive(Debug)]
#[repr(C, align(0x1000))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[inline(always)]
pub fn pml4_index(addr: u64) -> usize {
    ((addr >> 39) & 0x1FF) as usize
}

#[inline(always)]
pub fn pdpt_index(addr: u64) -> usize {
    ((addr >> 30) & 0x1FF) as usize
}

#[inline(always)]
pub fn pd_index(addr: u64) -> usize {
    ((addr >> 21) & 0x1FF) as usize
}

#[inline(always)]
pub fn pt_index(addr: u64) -> usize {
    ((addr >> 12) & 0x1FF) as usize
}

impl PageTable {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); 512],
        }
    }

    #[inline(always)]
    pub unsafe fn from_ptr(ptr: *mut PageTable) -> &'static mut Self {
        unsafe { &mut *ptr }
    }

    #[inline(always)]
    pub fn get_entry(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }

    #[inline(always)]
    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry) {
        self.entries[index] = entry;
    }

    #[inline(always)]
    pub fn clear_entry(&mut self, index: usize) {
        self.entries[index] = PageTableEntry::new();
    }

    #[inline(always)]
    pub fn clear_all(&mut self) {
        let mut index = 0;
        while index < 512 {
            self.clear_entry(index);
            index += 1;
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PageTableEntry(u64);

#[allow(clippy::wrong_self_convention)]
impl PageTableEntry {
    #[inline(always)]
    pub fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    // TODO: use these bits 6, 8, 9-11, 52-62 in the future.
    // Bit 6, Bit 8, Bits 9 to 11 and Bits 52 to 62 are
    // ingored by hardware and are avariable for OS use.

    #[inline(always)]
    pub fn is_present(self) -> bool {
        self.0 & 0b1 == 0b1
    }

    #[inline(always)]
    pub fn is_read_write(self) -> bool {
        self.0 & 0b10 == 0b10
    }

    #[inline(always)]
    pub fn is_user_accessible(self) -> bool {
        self.0 & 0b100 == 0b100
    }

    #[inline(always)]
    pub fn is_write_through(self) -> bool {
        self.0 & 0b1000 == 0b1000
    }

    #[inline(always)]
    pub fn is_cache_disabled(self) -> bool {
        self.0 & 0b10000 == 0b10000
    }

    #[inline(always)]
    pub fn is_accessed(self) -> bool {
        self.0 & 0b100000 == 0b100000
    }

    #[inline(always)]
    pub fn is_no_execute(self) -> bool {
        self.0 >> 63 == 0b1
    }

    #[inline(always)]
    fn get_addr(self) -> u64 {
        self.0 & 0x000f_ffff_ffff_f000
    }

    #[inline(always)]
    fn set_addr(&mut self, addr: u64) {
        self.0 = (self.0 & !0x000f_ffff_ffff_f000) | (addr & 0x000f_ffff_ffff_f000);
    }

    #[inline(always)]
    pub fn get_table(self) -> &'static PageTable {
        unsafe { &*(self.get_addr() as *const PageTable) }
    }

    #[inline(always)]
    pub fn get_table_mut(self) -> &'static mut PageTable {
        unsafe { &mut *(self.get_addr() as *mut PageTable) }
    }

    #[inline(always)]
    pub fn set_table(&mut self, table: *const PageTable) {
        self.set_addr(table as u64);
    }

    #[inline(always)]
    pub fn get_page(self) -> *mut u8 {
        self.get_addr() as *mut u8
    }

    #[inline(always)]
    pub fn set_page(&mut self, page: *mut u8) {
        self.set_addr(page as u64)
    }

    #[inline(always)]
    pub fn set_present(&mut self) {
        self.0 |= 0b1;
    }
    #[inline(always)]
    pub fn set_read_write(&mut self) {
        self.0 |= 0b10;
    }

    #[inline(always)]
    pub fn set_user_accessible(&mut self) {
        self.0 |= 0b100;
    }

    #[inline(always)]
    pub fn set_write_through(&mut self) {
        self.0 |= 0b1000;
    }

    #[inline(always)]
    pub fn set_cache_disabled(&mut self) {
        self.0 |= 0b10000;
    }

    #[inline(always)]
    pub fn set_accessed(&mut self) {
        self.0 |= 0b100000;
    }

    #[inline(always)]
    pub fn set_no_execute(&mut self) {
        self.0 |= (0b1 << 63);
    }

    #[inline(always)]
    pub fn clear_present(&mut self) {
        self.0 &= !0b1;
    }
    #[inline(always)]
    pub fn clear_read_write(&mut self) {
        self.0 &= !0b10;
    }

    #[inline(always)]
    pub fn clear_user_accessible(&mut self) {
        self.0 &= !0b100;
    }

    #[inline(always)]
    pub fn clear_write_through(&mut self) {
        self.0 &= !0b1000;
    }

    #[inline(always)]
    pub fn clear_cache_disabled(&mut self) {
        self.0 &= !0b10000;
    }

    #[inline(always)]
    pub fn clear_accessed(&mut self) {
        self.0 &= !0b100000;
    }

    #[inline(always)]
    pub fn clear_no_execute(&mut self) {
        self.0 &= !(0b1 << 63)
    }
}
