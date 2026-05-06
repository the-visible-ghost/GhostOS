#![allow(dead_code, unreachable_code)]
#![no_main]
#![no_std]

mod buffer;
mod shapes;
use buffer::Buffer;

use core::time::Duration;
// use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

fn draw_ui() -> Result<(), uefi::Error> {
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    let mut target_mode = None;

    for mode in gop.modes() {
        let mode = mode;
        let info = mode.info();
        let (w, h) = info.resolution();

        if w == 1920 && h == 1080 {
            target_mode = Some(mode);
        }
    }

    if let Some(mode) = target_mode {
        gop.set_mode(&mode).unwrap();
    }

    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);

    // for x in 0..width {
    //     for y in 0..height {
    //         let pixel = buffer.pixel(x, y).unwrap();
    //         pixel.red = (x % 255) as u8;
    //         pixel.green = ((x + y) % 255) as u8;
    //         pixel.blue = (y % 255) as u8;
    //     }
    // }

    shapes::conic::circle(
        &mut buffer,
        ((width as isize) / 2, (height as isize) / 2),
        200,
        (255, 0, 0),
    );

    buffer.blit(&mut gop);

    Ok(())
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    draw_ui();

    loop {
        boot::stall(Duration::from_secs(10));
    }

    Status::SUCCESS
}
