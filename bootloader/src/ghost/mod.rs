use uefi::{boot::ScopedProtocol, fs::FileSystem, proto::console::text};

use self::common::fs;

pub mod boot;
pub mod common;
pub mod config;
pub mod gfx;

pub struct Ghost<'a> {
    pub ih: uefi::Handle,
    pub gfx: Option<gfx::GhostGFX>,
    pub fs: FileSystem,
    pub cfg: config::Config<'a>,
    pub stdin: ScopedProtocol<text::Input>,
}

impl<'a> Ghost<'a> {
    pub fn init() -> Self {
        let image_handle = uefi::boot::image_handle();
        let stdin_handle = uefi::boot::get_handle_for_protocol::<text::Input>().unwrap();
        let stdin = uefi::boot::open_protocol_exclusive::<text::Input>(stdin_handle).unwrap();

        let mut file_sys = fs::init(image_handle).expect("Failed to initalize filesystem");

        let gfx = gfx::init(&mut file_sys);
        let cfg = config::load(&mut file_sys);

        Self {
            fs: file_sys,
            ih: image_handle,
            gfx,
            cfg,
            stdin,
        }
    }
}
