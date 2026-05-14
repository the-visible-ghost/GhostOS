extern crate alloc;
use alloc::string::String;

use ab_glyph::FontVec;
use hashbrown::HashMap;

pub struct Family {
    pub name: String,
    pub variants: HashMap<String, Variant>,
}

pub struct Variant {
    pub name: String,
    pub font: FontVec,
}
