use hashbrown::HashMap;
use uefi::{CStr16, cstr16, fs::FileSystem};

use super::font::Family;

pub struct Theme<'a> {
    pub name: &'a str,
    pub fonts: HashMap<&'a str, Family<'a>>,
    pub images: HashMap<&'a str, &'a [u8]>,
}

pub fn load<'a>(fs: &'a mut FileSystem, name: &'a str) -> Theme<'a> {
    let mut theme = Theme {
        name,
        fonts: HashMap::new(),
        images: HashMap::new(),
    };
    load_fonts(&mut theme, fs);
    theme
}

fn load_fonts(theme: &mut Theme, fs: &mut FileSystem) {
    fs.read_dir(cstr16!("\\ghost\\themes\\default\\assets\\fonts"));
}

fn load_font(_path: &CStr16) {}
