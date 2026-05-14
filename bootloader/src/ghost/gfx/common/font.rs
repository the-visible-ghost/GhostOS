extern crate alloc;
use alloc::string::String;

use super::theme;
use uefi::{CStr16, cstr16, fs::FileSystem};

use ab_glyph::FontVec;
use hashbrown::HashMap;

pub struct Family {
    pub name: String,
    pub variants: HashMap<String, Variant>,
}

struct Variant {
    pub name: String,
    pub font: FontVec,
}

pub fn load_fonts(theme: &mut theme::Theme, fs: &mut FileSystem) {
    return;
    // let path = format!("\\ghost\\themes\\{}\\assets\\fonts", theme.name);
    let mut path = theme.path.clone();
    path.push_str(cstr16!("\\assets\\fonts\\"));

    let dir = fs
        .read_dir(path.as_ref())
        .unwrap_or_else(|_| panic!("Failed to read fonts directory: {}", path));

    dir.for_each(|item| {
        let item = item.unwrap();
        if item.is_directory()
            && item.file_name() != cstr16!(".")
            && item.file_name() != cstr16!("..")
        {
            let mut font_path = path.clone();
            font_path.push_str(item.file_name());
            theme.fonts.insert(
                String::from(item.file_name()),
                load_font(fs, font_path.as_ref()),
            );
        }
    });
}

fn load_font(fs: &mut FileSystem, path: &CStr16) -> Family {
    let dir = fs
        .read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to read font: {}", path));

    let family = Family {
        name: todo!(),
        variants: todo!(),
    };

    family
}
