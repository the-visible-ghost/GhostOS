use crate::ghost;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use log::info;
use uefi::boot;
use uefi::cstr16;
use uefi::mem::memory_map::{MemoryMap, MemoryMapMut, MemoryType};

extern crate alloc;
use alloc::vec::Vec;

pub fn boot(g_host: &mut ghost::Ghost) {
    let fs = &mut g_host.fs;
    let buffer = fs
        .read(cstr16!("\\ghost-krnl"))
        .expect("Cant open \\ghost-krnl");
    let elf = ElfBytes::<AnyEndian>::minimal_parse(&buffer).expect("Invalid ELF");
    let base: u64 = 0x100000;
    let entry = (base + elf.ehdr.e_entry) as usize;
    let phdrs = elf.segments().expect("no segments");
    for ph in phdrs {
        let seg_start = ph.p_offset as usize;
        let seg_end = seg_start + ph.p_filesz as usize;
        let segment = &buffer[seg_start..seg_end];
        let dest = (base + ph.p_vaddr) as *mut u8;

        unsafe {
            core::ptr::copy_nonoverlapping(segment.as_ptr(), dest, segment.len());
        }
    }

    info!("Calling kernel ...");
    let gfx = g_host.gfx.as_mut().unwrap();
    let pitch = gfx.graphics_proto.current_mode_info().stride();
    let mut uefi_mmap = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
    uefi_mmap.sort(); // cuz why not
    let mut mmap_entries = Vec::with_capacity(uefi_mmap.len());
    for entry in uefi_mmap.entries() {
        mmap_entries.push(common::mmap::MemoryEntry::new(
            entry.ty.0,
            entry.phys_start,
            entry.virt_start,
            entry.page_count,
        ));
    }
    unsafe {
        let boot_info = common::BootInfo {
            framebuffer: common::gfx::buffer::Buffer::new(
                common::gfx::buffer::Resolution::new(gfx.resolution.0, gfx.resolution.1),
                pitch,
                core::slice::from_raw_parts_mut(
                    gfx.graphics_proto.frame_buffer().as_mut_ptr() as *mut u32,
                    gfx.resolution.1 * pitch,
                ),
            ),
            memory_map: common::mmap::MemoryMap::new(mmap_entries.as_ptr(), uefi_mmap.len()),
        };
        info!("{:?}", boot_info);

        // USE ITS VALUE (for now no)
        let _ = boot::exit_boot_services(None);

        let kernel: extern "sysv64" fn(&common::BootInfo) -> ! = core::mem::transmute(entry);
        kernel(&boot_info);
    }
}
