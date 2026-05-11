#![no_main]
#![no_std]

mod ghost;

use core::time::Duration;

use elf::ElfBytes;
use elf::endian::AnyEndian;
use log::info;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::{Char16, prelude::*};

use crate::ghost::gfx;

#[derive(Debug)]
#[repr(C)]
struct BootInfo {
    pub framebuffer_ptr: u64,
    pub framebuffer_width: u64,
    pub framebuffer_height: u64,
    pub framebuffer_pitch: u64, // pixels per row
}

fn boot_ghostos(g_host: &mut ghost::Ghost) {
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
        let bi = BootInfo {
            framebuffer_ptr: gfx.graphics_proto.frame_buffer().as_mut_ptr() as u64,
            framebuffer_width: gfx.resolution.0 as u64,
            framebuffer_height: gfx.resolution.1 as u64,
            framebuffer_pitch: gfx.graphics_proto.current_mode_info().stride() as u64,
        };
        info!("{:?}", bi);
        boot::exit_boot_services(None);
        let kernel: extern "sysv64" fn(boot_info: &BootInfo, width: u64) -> ! =
            core::mem::transmute(entry);
        kernel(&bi, 800);
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mut g_host = ghost::Ghost::init();
    if let Some(gfx) = &mut g_host.gfx {
        gfx::set_res(gfx, (1920, 1080));
    }

    let enter_key = Char16::try_from('\r').unwrap();
    let t_key = Char16::try_from('t').unwrap();
    let e_key = Char16::try_from('e').unwrap();

    loop {
        gfx::render(&mut g_host);
        if let Ok(key) = g_host.stdin.read_key()
            && let Some(key) = key
        {
            match key {
                Key::Printable(key) => {
                    if key == enter_key {
                        info!("Boot key pressed");
                        boot_ghostos(&mut g_host);
                        break;
                    } else if key == t_key {
                        info!("Terminal Key pressed");
                    } else if key == e_key {
                        info!("Edit Key pressed");
                    }
                }
                Key::Special(code) => match code {
                    ScanCode::UP | ScanCode::LEFT => {}
                    ScanCode::DOWN | ScanCode::RIGHT => {}
                    ScanCode::HOME => {}
                    ScanCode::END => {}
                    ScanCode::ESCAPE => {
                        break;
                    }
                    _ => {}
                },
            }
        }
        boot::stall(Duration::from_millis(10));
    }

    Status::SUCCESS
}
