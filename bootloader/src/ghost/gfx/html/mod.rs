extern crate alloc;

use alloc::vec::Vec;

// mod adapter;
// mod parser;

pub enum HtmlTag {
    BODY,
    DIV,
    IMG,
    VIDEO,
    INPUT,
    SPAN,
    CODE,
}

pub struct Element<'a> {
    tag: HtmlTag,
    attributes: Vec<Attribute<'a>>,
    children: Vec<Element<'a>>,
}

pub enum Attribute<'a> {
    Id(&'a str),
    Class(&'a str),
    Src(&'a str),
    Autoplay,
    Width(usize),
    Height(usize),
}
