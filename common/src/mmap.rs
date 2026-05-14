#[repr(C)]
#[derive(Debug)]
pub struct MemoryMap {
    entries: *const MemoryEntry,
    num_entries: usize,
}

impl MemoryMap {
    pub fn new(entries: *const MemoryEntry, num_entries: usize) -> Self {
        Self {
            entries,
            num_entries,
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn entries(&self) -> &[MemoryEntry] {
        unsafe { core::slice::from_raw_parts(self.entries, self.num_entries) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryEntry {
    pub memorytype: u32,
    pub phys_start: u64,
    pub virt_start: u64,
    pub page_count: u64,
    // attributes: MemoryAttribute,
}

impl MemoryEntry {
    pub fn new(memorytype: u32, phys_start: u64, virt_start: u64, page_count: u64) -> Self {
        Self {
            memorytype,
            phys_start,
            virt_start,
            page_count,
        }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
/// This is partially taken from uefi crate...
pub enum MemoryType {
    RESERVED = 0,
    /// The code portions of a loaded UEFI application.
    LOADER_CODE = 1,
    /// The data portions of a loaded UEFI applications,
    /// as well as any memory allocated by it.
    LOADER_DATA = 2,
    /// Code of the boot drivers.
    ///
    /// Can be reused after OS is loaded.
    BOOT_SERVICES_CODE = 3,
    /// Memory used to store boot drivers' data.
    ///
    /// Can be reused after OS is loaded.
    BOOT_SERVICES_DATA = 4,
    /// Runtime drivers' code.
    RUNTIME_SERVICES_CODE = 5,
    /// Runtime services' code.
    RUNTIME_SERVICES_DATA = 6,
    /// Free usable memory.
    CONVENTIONAL = 7,
    /// Memory in which errors have been detected.
    UNUSABLE = 8,
    /// Memory that holds ACPI tables.
    /// Can be reclaimed after they are parsed.
    ACPI_RECLAIM = 9,
    /// Firmware-reserved addresses.
    ACPI_NON_VOLATILE = 10,
    /// A region used for memory-mapped I/O.
    MMIO = 11,
    /// Address space used for memory-mapped port I/O.
    MMIO_PORT_SPACE = 12,
    /// Address space which is part of the processor.
    PAL_CODE = 13,
    /// Memory region which is usable and is also non-volatile.
    PERSISTENT_MEMORY = 14,
    /// Memory that must be accepted by the boot target before it can be used.
    UNACCEPTED = 15,
    /// End of the defined memory types. Higher values are possible though, see
    /// [`MemoryType::RESERVED_FOR_OEM`] and [`MemoryType::RESERVED_FOR_OS_LOADER`].
    MAX = 16,

    RESERVED_FOR_OEM(u32),
    RESERVED_FOR_OS_LOADER(u32),
    CUSTOM(u32),
}

impl MemoryType {
    pub fn from_u32(ty: u32) -> Option<Self> {
        // ik m leaving some stuff will do when needed
        match ty {
            0 => Some(MemoryType::RESERVED),
            1 => Some(MemoryType::LOADER_CODE),
            2 => Some(MemoryType::LOADER_DATA),
            3 => Some(MemoryType::BOOT_SERVICES_CODE),
            4 => Some(MemoryType::BOOT_SERVICES_DATA),
            5 => Some(MemoryType::RUNTIME_SERVICES_CODE),
            6 => Some(MemoryType::RUNTIME_SERVICES_DATA),
            7 => Some(MemoryType::CONVENTIONAL),
            8 => Some(MemoryType::UNUSABLE),
            9 => Some(MemoryType::ACPI_RECLAIM),
            10 => Some(MemoryType::ACPI_NON_VOLATILE),
            11 => Some(MemoryType::MMIO),
            12 => Some(MemoryType::MMIO_PORT_SPACE),
            13 => Some(MemoryType::PAL_CODE),
            14 => Some(MemoryType::PERSISTENT_MEMORY),
            15 => Some(MemoryType::UNACCEPTED),
            16 => Some(MemoryType::MAX),
            _ => None,
        }
    }
}
