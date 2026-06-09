#[derive(Debug)]
pub struct MemoryMap {
    pub entries: [MemoryRegion],
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub mem_class: MemoryClass,
    pub phys_start: usize,
    pub page_count: usize,
}

#[derive(Debug)]
pub enum MemoryClass {
    Reserved,
    Reclaimable,
    Free,
}
