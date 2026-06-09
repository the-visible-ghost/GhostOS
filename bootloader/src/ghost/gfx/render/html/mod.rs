use alloc::{string::String, vec::Vec};
use hashbrown::HashMap;

mod lexer;
mod parser;
mod token;

#[allow(unused_variables)]
pub fn parse(data: &[u8]) -> Node {
    parser::Parser::new(lexer::Lexer::new(data)).parse()
}

#[derive(Debug)]
pub enum Node {
    Element {
        tag: HtmlTag,
        attributes: AttrMap,
        children: Vec<Node>,
    },
    Text(String),
}

impl Node {
    pub fn get_element_by_id(&self, target: &str) -> Option<&Node> {
        match self {
            Node::Element {
                tag: _,
                attributes,
                children,
            } => {
                if let Some(id) = attributes.get("id")
                    && id == target
                {
                    return Some(self);
                }
                for child in children {
                    if let Some(node) = child.get_element_by_id(target) {
                        return Some(node);
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn get_elements_by_class_name(&self, target: &str) -> Vec<&Node> {
        let mut elems = Vec::<&Node>::new();
        if let Node::Element {
            tag: _,
            attributes,
            children,
        } = self
        {
            if let Some(classes) = attributes.get("class")
                && classes.split_whitespace().any(|class| class == target)
            {
                elems.push(self);
            }

            for child in children {
                elems.append(&mut child.get_elements_by_class_name(target));
            }
        }
        elems
    }
}

#[derive(Debug)]
pub enum HtmlTag {
    Html,
    Body,
    Div,
    Img,
    Video,
    Input,
    Span,
    Code,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
}

type AttrMap = HashMap<String, String>;
