#[repr(C)]
#[derive(Debug)]
pub struct MemoryMap {
    pub entries: *const MemoryEntry,
    pub num_entries: usize,
}

impl MemoryMap {
    pub fn new(entries: *const MemoryEntry, num_entries: usize) -> Self {
        Self {
            entries,
            num_entries,
        }
    }

    #[inline]
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// This is partially taken from uefi crate...
pub enum MemoryType {
    RESERVED,
    /// The code portions of a loaded UEFI application.
    LOADER_CODE,
    /// The data portions of a loaded UEFI applications,
    /// as well as any memory allocated by it.
    LOADER_DATA,
    /// Code of the boot drivers.
    ///
    /// Can be reused after OS is loaded.
    BOOT_SERVICES_CODE,
    /// Memory used to store boot drivers' data.
    ///
    /// Can be reused after OS is loaded.
    BOOT_SERVICES_DATA,
    /// Runtime drivers' code.
    RUNTIME_SERVICES_CODE,
    /// Runtime services' code.
    RUNTIME_SERVICES_DATA,
    /// Free usable memory.
    CONVENTIONAL,
    /// Memory in which errors have been detected.
    UNUSABLE,
    /// Memory that holds ACPI tables.
    /// Can be reclaimed after they are parsed.
    ACPI_RECLAIM,
    /// Firmware-reserved addresses.
    ACPI_NON_VOLATILE,
    /// A region used for memory-mapped I/O.
    MMIO,
    /// Address space used for memory-mapped port I/O.
    MMIO_PORT_SPACE,
    /// Address space which is part of the processor.
    PAL_CODE,
    /// Memory region which is usable and is also non-volatile.
    PERSISTENT_MEMORY,
    /// Memory that must be accepted by the boot target before it can be used.
    UNACCEPTED,
    /// End of the defined memory types. Higher values are possible though, see
    /// [`MemoryType::RESERVED_FOR_OEM`] and [`MemoryType::RESERVED_FOR_OS_LOADER`].
    MAX,

    RESERVED_FOR_OEM(u32),
    RESERVED_FOR_OS_LOADER(u32),
    CUSTOM(u32),
}

const MMAP_TYPES: [MemoryType; 17] = [
    MemoryType::RESERVED,
    MemoryType::LOADER_CODE,
    MemoryType::LOADER_DATA,
    MemoryType::BOOT_SERVICES_CODE,
    MemoryType::BOOT_SERVICES_DATA,
    MemoryType::RUNTIME_SERVICES_CODE,
    MemoryType::RUNTIME_SERVICES_DATA,
    MemoryType::CONVENTIONAL,
    MemoryType::UNUSABLE,
    MemoryType::ACPI_RECLAIM,
    MemoryType::ACPI_NON_VOLATILE,
    MemoryType::MMIO,
    MemoryType::MMIO_PORT_SPACE,
    MemoryType::PAL_CODE,
    MemoryType::PERSISTENT_MEMORY,
    MemoryType::UNACCEPTED,
    MemoryType::MAX,
];

impl MemoryType {
    #[inline]
    pub fn from_u32(ty: u32) -> Self {
        // ik m leaving some stuff will do when needed
        // match ty {
        //     0 => MemoryType::RESERVED,
        //     1 => MemoryType::LOADER_CODE,
        //     2 => MemoryType::LOADER_DATA,
        //     3 => MemoryType::BOOT_SERVICES_CODE,
        //     4 => MemoryType::BOOT_SERVICES_DATA,
        //     5 => MemoryType::RUNTIME_SERVICES_CODE,
        //     6 => MemoryType::RUNTIME_SERVICES_DATA,
        //     7 => MemoryType::CONVENTIONAL,
        //     8 => MemoryType::UNUSABLE,
        //     9 => MemoryType::ACPI_RECLAIM,
        //     10 => MemoryType::ACPI_NON_VOLATILE,
        //     11 => MemoryType::MMIO,
        //     12 => MemoryType::MMIO_PORT_SPACE,
        //     13 => MemoryType::PAL_CODE,
        //     14 => MemoryType::PERSISTENT_MEMORY,
        //     15 => MemoryType::UNACCEPTED,
        //     16 => MemoryType::MAX,
        //     _ => MemoryType::CUSTOM(ty),
        // }

        if ty <= 16 {
            return MMAP_TYPES[ty as usize];
        }
        MemoryType::CUSTOM(ty)
    }
}
