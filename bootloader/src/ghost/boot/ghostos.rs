use crate::ghost;
use common;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use log::info;
use uefi::boot;
use uefi::cstr16;

pub fn boot(g_host: &mut ghost::Ghost) {
    let fs = g_host.fs.as_mut().unwrap();
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

    unsafe {
        info!("Calling kernel ...");
        let gfx = g_host.gfx.as_mut().unwrap();
        // let bi = BootInfo {
        //     framebuffer_ptr: gfx.graphics_proto.frame_buffer().as_mut_ptr() as u64,
        //     framebuffer_width: gfx.resolution.0 as u64,
        //     framebuffer_height: gfx.resolution.1 as u64,
        //     framebuffer_pitch: gfx.graphics_proto.current_mode_info().stride() as u64,
        // };
        let pitch = gfx.graphics_proto.current_mode_info().stride();
        let boot_info = common::BootInfo {
            framebuffer: common::gfx::buffer::Buffer::new(
                common::gfx::buffer::Resolution::new(gfx.resolution.0, gfx.resolution.1),
                pitch,
                core::slice::from_raw_parts_mut(
                    gfx.graphics_proto.frame_buffer().as_mut_ptr() as *mut u32,
                    gfx.resolution.1 * pitch,
                ),
            ),
        };
        info!("{:?}", boot_info);
        boot::exit_boot_services(None);
        let kernel: extern "sysv64" fn(&common::BootInfo) -> ! = core::mem::transmute(entry);
        kernel(&boot_info);
    }
}
