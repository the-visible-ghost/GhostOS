use alloc::vec::Vec;
use elf::{ElfBytes, endian::AnyEndian};
use uefi::mem::memory_map::{MemoryMap, MemoryMapMut};
use uefi::{boot, cstr16};

use crate::ghost::Ghost;

pub fn boot(ghost: &mut Ghost) {
    // reading elf
    let fs = &mut ghost.fs;
    let buffer = fs
        .read(cstr16!("\\ghost-krnl"))
        .expect("Cant open \\ghost-krnl");
    let elf =
        ElfBytes::<AnyEndian>::minimal_parse(buffer.as_ref()).expect("Cannot parse ELF headers");
    let phdrs = elf.segments().expect("ELF contains no segments");

    // headers adapter
    let mut prog_headers = Vec::with_capacity(elf.ehdr.e_phnum as usize);
    let mut kernel_entry = elf.ehdr.e_entry;
    let mut set_kernel_offset = false;

    // allocating and filling adapter
    for header in phdrs {
        if header.p_type != elf::abi::PT_LOAD {
            continue;
        }

        let pages = boot::allocate_pages(
            boot::AllocateType::AnyPages,
            boot::MemoryType::LOADER_DATA,
            header.p_memsz.div_ceil(4096) as usize,
        )
        .expect("Cannot allocate pages");

        if !set_kernel_offset {
            set_kernel_offset = true;
            kernel_entry -= header.p_vaddr - pages.as_ptr() as u64;
        }

        let segment =
            &buffer[header.p_offset as usize..(header.p_offset + header.p_filesz) as usize];

        unsafe { core::ptr::copy_nonoverlapping(segment.as_ptr(), pages.as_ptr(), segment.len()) };

        prog_headers.push(common::phdrs::ProgramHeader {
            p_type: common::phdrs::HeaderType::LOAD,
            offset: header.p_offset,
            file_size: header.p_filesz,
            mem_size: header.p_memsz,
            virt_addr: header.p_vaddr,
            phys_addr: pages.as_ptr() as u64,
            align: header.p_align,
            flags: header.p_flags,
        });
    }

    // Adapting UEFI memory map
    let mut uefi_mmap =
        boot::memory_map(boot::MemoryType::LOADER_DATA).expect("Cant get UEFI memory map");
    uefi_mmap.sort();
    let mut mmap_entries = Vec::with_capacity(uefi_mmap.len());
    for entry in uefi_mmap.entries() {
        mmap_entries.push(common::mmap::MemoryEntry::new(
            entry.ty.0,
            entry.phys_start,
            entry.virt_start,
            entry.page_count,
        ));
    }

    // Getting Final Variables
    let gfx = ghost.gfx.as_mut().unwrap();
    let memory_map = common::mmap::MemoryMap::new(mmap_entries.as_ptr(), mmap_entries.len());
    let framebuffer = common::gfx::buffer::Buffer::new(
        common::gfx::buffer::Resolution::new(gfx.resolution.0, gfx.resolution.1),
        gfx.graphics_proto.current_mode_info().stride(),
        unsafe {
            core::slice::from_raw_parts_mut(
                gfx.graphics_proto.frame_buffer().as_mut_ptr() as *mut u32,
                gfx.resolution.1 * gfx.graphics_proto.current_mode_info().stride(),
            )
        },
    );

    log::debug!("prog_headers = {:?}", prog_headers);
    log::debug!(
        "framebuffer = {:X}",
        gfx.graphics_proto.frame_buffer().as_mut_ptr() as u64
    );

    let boot_info = common::BootInfo {
        framebuffer,
        memory_map,
        prog_headers: &mut common::phdrs::Headers {
            ptr: prog_headers.as_mut_ptr(),
            len: prog_headers.len() as u64,
        } as *mut common::phdrs::Headers,
    };

    // Launching kernel
    unsafe {
        let kernel: extern "sysv64" fn(&common::BootInfo) -> ! = core::mem::transmute(kernel_entry);
        log::info!("Exiting boot services ...\r\n--- --- --- Launching GhostOS Kernel --- --- ---");

        let _ = boot::exit_boot_services(None);
        kernel(&boot_info);
    }
}
