use hashbrown::HashMap;

use super::font::Family;

pub struct Theme<'a> {
    pub name: &'a str,
    pub fonts: HashMap<&'a str, Family<'a>>,
    pub images: HashMap<&'a str, &'a [u8]>,
}

pub fn load(name: &str) -> Theme {
    let mut theme = Theme {
        name,
        fonts: HashMap::new(),
        images: HashMap::new(),
    };
    load_fonts(&mut theme);
    theme
}

fn load_fonts(theme: &mut Theme) {
    theme;
}
