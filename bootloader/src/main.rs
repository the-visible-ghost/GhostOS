#![no_std]
#![no_main]

mod ghost;

use core::time::Duration;

use log::info;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::{Char16, prelude::*};

use crate::ghost::gfx::render_tree;

fn keypress_handler(g_host: &mut ghost::Ghost) -> u32 {
    let enter_key = Char16::try_from('\r').unwrap();
    let t_key = Char16::try_from('t').unwrap();
    let s_key = Char16::try_from('s').unwrap();
    let e_key = Char16::try_from('e').unwrap();

    if let Ok(key) = g_host.stdin.read_key()
        && let Some(key) = key
    {
        match key {
            Key::Printable(key) => {
                if key == enter_key {
                    info!("BOOT key pressed");
                    ghost::boot::ghostos::boot(g_host);
                    return 1;
                } else if key == t_key {
                    info!("TERMINAL key pressed");
                    ghost::boot::chainload::boot(g_host, cstr16!("\\test\\ghost.efi"))
                } else if key == e_key {
                    info!("EDIT key pressed");
                } else if key == s_key {
                    info!("SCREENSHOT key pressed");
                }
            }
            Key::Special(code) => match code {
                ScanCode::UP | ScanCode::LEFT => {
                    info!("UP or LEFT key pressed");
                }
                ScanCode::DOWN | ScanCode::RIGHT => {
                    info!("DOWN or RIGHT key pressed");
                }
                ScanCode::HOME => {
                    info!("HOME key pressed");
                }
                ScanCode::END => {
                    info!("END key pressed");
                }
                ScanCode::ESCAPE => {
                    info!("ESCAPE key pressed");
                    return 1;
                }
                _ => {}
            },
        }
    };
    0
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mut g_host = ghost::Ghost::init();
    info!("Ghost initialized successfully ...");

    if let Some(gfx) = &mut g_host.gfx {
        gfx.set_res(1920, 1080);
        // gfx.frame_buffer.pixels[0] = BltPixel::new(255, 0, 0);
        // gfx.frame_buffer.blit(&mut gfx.graphics_proto);
    }

    let html = g_host
        .fs
        .read_to_string(cstr16!("\\ghost\\themes\\default\\index.html"))
        .unwrap();

    render_tree::html::parse(html);
    info!("Render tree parsing complete");

    loop {
        if let Some(gfx) = &mut g_host.gfx {
            gfx.render();
        }
        let s = keypress_handler(&mut g_host);
        if s == 1 {
            break;
        }
        boot::stall(Duration::from_millis(10));
    }

    Status::SUCCESS
}
