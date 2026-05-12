#![no_std]
#![no_main]

mod ghost;

use core::time::Duration;

use log::info;
use uefi::proto::console::gop::BltPixel;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::{Char16, prelude::*};

use crate::ghost::gfx;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mut g_host = ghost::Ghost::init();
    if let Some(gfx) = &mut g_host.gfx {
        gfx::set_res(gfx, (1920, 1080));
        gfx.frame_buffer.pixels[0] = BltPixel::new(255, 0, 0);
        gfx.frame_buffer.blit(&mut gfx.graphics_proto);
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
                        ghost::boot::ghostos::boot(&mut g_host);
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
