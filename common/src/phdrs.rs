// Program Hedares representation
use core::fmt;

#[repr(C)]
pub struct Headers {
    pub ptr: *mut ProgramHeader,
    pub len: u64,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum HeaderType {
    LOAD,
}

#[repr(C)]
pub struct ProgramHeader {
    pub p_type: HeaderType,
    pub offset: u64,

    pub file_size: u64,
    pub mem_size: u64,

    pub virt_addr: u64,
    pub phys_addr: u64,

    pub align: u64,
    pub flags: u32,
}

impl fmt::Debug for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProgramHeader")
            .field("p_type", &format_args!("{:?}", self.p_type))
            .field("offset", &format_args!("{:#018X}", self.offset))
            .field("file_size", &format_args!("{:#018X}", self.file_size))
            .field("mem_size", &format_args!("{:#018X}", self.mem_size))
            .field("virt_addr", &format_args!("{:#018X}", self.virt_addr))
            .field("phys_addr", &format_args!("{:#018X}", self.phys_addr))
            .field("align", &format_args!("{:#018X}", self.align))
            .field("flags", &format_args!("{:#010X}", self.flags))
            .finish()
    }
}
