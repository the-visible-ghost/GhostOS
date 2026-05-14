extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

mod lexer;
mod parser;

#[allow(clippy::upper_case_acronyms)]
pub enum HtmlTag {
    BODY,
    DIV,
    IMG,
    VIDEO,
    INPUT,
    SPAN,
    CODE,
}

pub struct Element<'element> {
    tag: HtmlTag,
    attributes: Vec<Attribute<'element>>,
    children: Vec<Element<'element>>,
}

pub enum Attribute<'a> {
    Id(&'a str),
    Class(&'a str),
    Src(&'a str),
    Autoplay,
    Width(usize),
    Height(usize),
}

pub fn parse<'a>(html: String) -> Element<'a> {
    let data = html.as_str();
    let mut lxr = lexer::Lexer::new(data.as_bytes());
    todo!()
}
