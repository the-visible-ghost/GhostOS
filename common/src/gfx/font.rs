use ab_glyph::FontRef;
use hashbrown::HashMap;

pub struct Family<'a> {
    pub name: &'a str,
    pub variants: HashMap<&'a str, Variant<'a>>,
}

pub struct Variant<'a> {
    pub name: &'a str,
    pub font: FontRef<'a>,
}
