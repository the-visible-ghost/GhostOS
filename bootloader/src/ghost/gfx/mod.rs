// use log::info;
use uefi::{
    boot::{self, ScopedProtocol},
    proto::console::gop::GraphicsOutput,
};

use self::common::buffer::Buffer;
use super::Ghost;

mod common;
mod html;

pub struct GhostGFX {
    pub frame_count: u64,
    pub frame_buffer: Buffer,
    pub resolution: (usize, usize),
    pub graphics_proto: ScopedProtocol<GraphicsOutput>,
    // pub theme: common::theme::Theme<'a>,
}

pub fn init<'a>() -> Option<GhostGFX> {
    if let Ok(graphics_handle) = boot::get_handle_for_protocol::<GraphicsOutput>()
        && let Ok(gop) = boot::open_protocol_exclusive::<GraphicsOutput>(graphics_handle)
    {
        let res = gop.current_mode_info().resolution();
        return Some(GhostGFX {
            frame_count: 0,
            resolution: res,
            frame_buffer: Buffer::new(res.0, res.1),
            graphics_proto: gop,
            // theme: common::theme::load(ghost.fs.as_mut().unwrap(), "default"),
        });
    }
    None
}

pub fn set_res(gfx: &mut GhostGFX, res: (usize, usize)) {
    let mut target_mode = None;
    for mode in gfx.graphics_proto.modes() {
        let mode = mode;
        let info = mode.info();
        let (w, h) = info.resolution();

        if w == res.0 && h == res.1 {
            target_mode = Some(mode);
        }
    }
    if let Some(mode) = target_mode {
        gfx.graphics_proto.set_mode(&mode).unwrap();
        gfx.frame_buffer.resize(res.0, res.1);
        gfx.resolution = res;
    }
}

pub fn render(ghost: &mut Ghost) {
    if let Some(gfx) = &mut ghost.gfx {
        // info!("frame drawn");
        gfx.frame_buffer.blit(&mut gfx.graphics_proto);
        gfx.frame_count += 1;
    }
}
