use self::common::buffer::Buffer;
use self::render::Engine;
use uefi::{
    CString16,
    boot::{self, ScopedProtocol},
    cstr16,
    fs::FileSystem,
    proto::console::gop::GraphicsOutput,
};

mod common;
pub mod render;
mod state;

pub struct GhostGFX {
    pub frame_buffer: Buffer,
    pub resolution: (usize, usize),
    pub graphics_proto: ScopedProtocol<GraphicsOutput>,
    pub theme: common::theme::Theme,
    pub ui_state: state::State,
}

pub fn init(fs: &mut FileSystem) -> Option<GhostGFX> {
    if let Ok(graphics_handle) = boot::get_handle_for_protocol::<GraphicsOutput>()
        && let Ok(gop) = boot::open_protocol_exclusive::<GraphicsOutput>(graphics_handle)
    {
        let res = gop.current_mode_info().resolution();
        let theme = common::theme::load(fs, CString16::from(cstr16!("default")));
        return Some(GhostGFX {
            theme,
            resolution: res,
            frame_buffer: Buffer::new(res.0, res.1),
            graphics_proto: gop,
            ui_state: state::State { frame_count: 0 },
        });
    }
    None
}

impl GhostGFX {
    pub fn set_res(&mut self, width: usize, height: usize) -> bool {
        let mut target_mode = None;
        for mode in self.graphics_proto.modes() {
            let info = mode.info();
            let (w, h) = info.resolution();

            if w == width && h == height {
                target_mode = Some(mode);
            }
        }
        if let Some(mode) = target_mode {
            self.graphics_proto.set_mode(&mode).unwrap();
            self.frame_buffer.resize(width, height);
            self.resolution = (width, height);
            return true;
        }
        false
    }

    pub fn render(&mut self, engine: &mut Engine) {
        self.ui_state.increment();
        // engine.render(&mut self.ui_state);
        let _ = self.frame_buffer.blit(&mut self.graphics_proto);
    }
}
