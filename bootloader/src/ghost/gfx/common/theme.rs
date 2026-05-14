extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::font;
use super::imgs;
use hashbrown::HashMap;
use uefi::{CString16, fs::FileSystem};

pub struct Theme {
    pub path: CString16,
    pub fonts: HashMap<String, font::Family>,
    pub _images: HashMap<String, Vec<u8>>,
}

pub fn load(fs: &mut FileSystem, path: CString16) -> Theme {
    let mut theme = Theme {
        path,
        fonts: HashMap::new(),
        _images: HashMap::new(),
    };
    font::load_fonts(&mut theme, fs);
    imgs::load_images(&mut theme, fs);
    theme
}
