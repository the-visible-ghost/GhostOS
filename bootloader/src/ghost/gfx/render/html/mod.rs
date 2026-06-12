use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashMap;

mod lexer;
mod parser;
mod token;

#[allow(unused_variables)]
pub fn parse(data: &[u8]) -> Node {
    parser::Parser::new(lexer::Lexer::new(data)).parse()
}

#[derive(Debug, Clone)]
pub enum Node {
    Element {
        tag: HtmlTag,
        node_id: u16,
        attributes: AttrMap,
        children: Vec<Node>,
    },
    Text(String),
}

impl Node {
    pub fn node_id(&self) -> u16 {
        match self {
            Node::Element { node_id, .. } => *node_id,
            _ => 0,
        }
    }

    pub fn clear_node_ids(&mut self) {
        if let Node::Element {
            node_id, children, ..
        } = self
        {
            *node_id = 0;
            children.iter_mut().for_each(|child| child.clear_node_ids());
        }
    }

    pub fn get_element_by_id(&mut self, target: &str) -> Option<&mut Node> {
        if let Node::Element { attributes, .. } = self {
            if attributes.get("id").is_some_and(|id| id == target) {
                return Some(self);
            }

            let children = match self {
                Node::Element { children, .. } => children,
                _ => return None,
            };

            for child in children {
                if let Some(found) = child.get_element_by_id(target) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn get_element_by_ghostid(&mut self, target: u16) -> Option<&mut Node> {
        if let Node::Element { node_id, .. } = self {
            if *node_id == target {
                return Some(self);
            }

            let children = match self {
                Node::Element { children, .. } => children,
                _ => return None,
            };

            for child in children {
                if let Some(found) = child.get_element_by_ghostid(target) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn get_first_element_by_class_name(&mut self, target: &str) -> Option<&mut Node> {
        if let Node::Element { .. } = self {
            if self.class_list().any(|c| c == target) {
                return Some(self);
            }

            let children = match self {
                Node::Element { children, .. } => children,
                _ => return None,
            };

            for child in children {
                if let Some(found) = child.get_element_by_id(target) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn get_elements_by_class_name(&self, target: &str) -> Vec<&Node> {
        let mut elems = Vec::<&Node>::new();
        if let Node::Element {
            attributes,
            children,
            ..
        } = self
        {
            if let Some(classes) = attributes.get("class")
                && classes.split_whitespace().any(|c| c == target)
            {
                elems.push(self);
            }

            for child in children {
                elems.extend(child.get_elements_by_class_name(target));
            }
        }
        elems
    }

    pub fn append_child(&mut self, node: Node) {
        if let Node::Element { children, .. } = self {
            children.push(node);
        }
    }

    pub fn remove_child(&mut self, node: &Node) {
        let node_id = node.node_id();
        if let Node::Element { children, .. } = self
            && let Some(pos) = children.iter().position(|child| child.node_id() == node_id)
        {
            children.remove(pos);
        }
    }

    pub fn clear_children(&mut self) {
        if let Node::Element { children, .. } = self {
            children.clear();
        }
    }

    pub fn set_attribute<K, V>(&mut self, attr: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        if let Node::Element { attributes, .. } = self {
            attributes.insert(attr.into(), value.into());
        }
    }

    pub fn class_list(&self) -> core::str::SplitWhitespace<'_> {
        if let Node::Element { attributes, .. } = self
            && let Some(classes) = attributes.get("class")
        {
            classes.split_whitespace()
        } else {
            "".split_whitespace()
        }
    }

    pub fn class_add(&mut self, class: &str) {
        if let Node::Element { attributes, .. } = self {
            if let Some(classes) = attributes.get_mut("class") {
                if !classes.split_whitespace().any(|c| c == class) {
                    classes.push(' ');
                    classes.push_str(class);
                }
            } else {
                attributes.insert("class".to_string(), class.to_string());
            }
        }
    }

    pub fn class_remove(&mut self, class: &str) {
        if let Node::Element { attributes, .. } = self
            && let Some(classes) = attributes.get_mut("class")
        {
            let mut result = String::new();

            for c in classes.split_whitespace() {
                if c != class {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(c);
                }
            }

            if result.is_empty() {
                attributes.remove("class");
            } else {
                *classes = result;
            }
        }
    }

    pub fn class_toggle(&mut self, class: &str) {
        if self.class_list().any(|c| c == class) {
            self.class_remove(class);
        } else {
            self.class_add(class);
        }
    }
}

#[derive(Debug, Clone)]
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
