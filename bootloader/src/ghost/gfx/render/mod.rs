use alloc::string::String;
use alloc::vec::Vec;

use crate::ghost::gfx::state::State;

mod css;
mod html;

pub struct RawData {
    pub html: String,
    pub css: String,
}

pub struct Engine {
    dom_tree: html::Node,
    stylesheet: Vec<css::Stylesheet>,
}

impl Engine {
    pub fn new(data: RawData) -> Self {
        let htmldom = html::parse(data.html.as_bytes());
        let cssom = css::parse(data.css.as_bytes());

        Self {
            dom_tree: htmldom,
            stylesheet: cssom,
        }
    }

    pub fn render(&mut self, _ui_state: &mut State) {
        // TODO: render
        todo!()
    }
}
