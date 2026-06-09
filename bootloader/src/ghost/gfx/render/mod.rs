use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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
        let mut menu: Vec<String> = vec![
            "Ghost OS".to_string(),
            "Arch Linux".to_string(),
            "Windows 11".to_string(),
        ];
        // let node = htmldom.get_element_by_id("main-menu");
        let nodes = htmldom.get_elements_by_class_name("item");
        log::info!("{:?}", nodes);
        Self {
            dom_tree: htmldom,
            stylesheet: cssom,
        }
    }

    pub fn render(&mut self) {
        // TODO: render
        todo!()
    }
}
