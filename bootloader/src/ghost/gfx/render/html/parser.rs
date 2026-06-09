use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::ghost::gfx::render::html::HtmlTag;

use super::Node;
use super::lexer::Lexer;
use super::token::{Token, TokenKind};

pub struct Parser<'parser> {
    lexer: Lexer<'parser>,
    last_token: Option<Token<'parser>>,
    current: Option<Token<'parser>>,
}

pub enum Tag {
    Opening(Node),
    Closing,
}

impl<'parser> Parser<'parser> {
    pub fn new(lexer: Lexer<'parser>) -> Self {
        let mut parser = Self {
            lexer,
            current: None,
            last_token: None,
        };
        parser.advance();
        parser
    }

    #[inline]
    fn advance(&mut self) {
        self.last_token = self.current;
        self.current = self.lexer.next_token();
    }

    #[inline]
    fn peek(&mut self) -> Option<Token<'parser>> {
        self.lexer.peek_token()
    }

    #[inline]
    fn expect_and_consume(&mut self, kind: TokenKind) {
        self.expect(kind);
        self.advance();
    }

    #[inline]
    fn expect(&mut self, kind: TokenKind) {
        let current = self.current.as_ref().unwrap();
        if current.kind() != kind {
            panic!("Expected {:?}, found {:?}", kind, current.kind());
        }
    }

    pub fn parse(&mut self) -> Node {
        match (self.current, self.peek()) {
            (Some(Token::AngleOpen), Some(Token::Ident(b"body"))) => match self.parse_tag() {
                Tag::Opening(node) => node,
                Tag::Closing => unreachable!(),
            },
            _ => panic!("The index.html file should start with a <body> tag"),
        }
    }

    #[inline]
    fn parse_tag(&mut self) -> Tag {
        self.expect_and_consume(TokenKind::AngleOpen);
        if matches!(self.current, Some(Token::Slash)) {
            self.advance(); // Consume slash
            self.advance(); // Consume tag name
            self.advance(); // Consume AngleCloe
            return Tag::Closing;
        }

        let tag = match self.current {
            Some(Token::Ident(b"body")) => HtmlTag::Body,
            Some(Token::Ident(b"div")) => HtmlTag::Div,
            Some(Token::Ident(b"image")) => HtmlTag::Img,
            Some(Token::Ident(b"video")) => HtmlTag::Video,
            Some(Token::Ident(b"input")) => HtmlTag::Input,
            Some(Token::Ident(b"span")) => HtmlTag::Span,
            Some(Token::Ident(b"code")) => HtmlTag::Code,
            Some(Token::Ident(b"h1")) => HtmlTag::H1,
            Some(Token::Ident(b"h2")) => HtmlTag::H2,
            Some(Token::Ident(b"h3")) => HtmlTag::H3,
            Some(Token::Ident(b"h4")) => HtmlTag::H4,
            Some(Token::Ident(b"h5")) => HtmlTag::H5,
            Some(Token::Ident(b"h6")) => HtmlTag::H6,
            Some(Token::Ident(b"p")) => HtmlTag::P,
            Some(Token::Ident(tag)) => panic!("Unknown tag {}", str::from_utf8(tag).unwrap()),
            _ => panic!("unclosed < at the end"),
        };
        self.advance(); // Consome tag

        let mut attributes: HashMap<String, String> = HashMap::new();
        while let Some(token) = self.current {
            match token {
                Token::AngleClose => {
                    self.advance(); // Consume angle close
                    break;
                }
                Token::Ident(name) => attributes.insert(
                    str::from_utf8(name).expect("Invalid UTF-8").to_string(),
                    self.parse_attr(),
                ),
                _ => panic!("Unexpected token {:?}", token),
            };
            self.advance();
        }

        let mut children = Vec::<Node>::new();
        loop {
            match self.current {
                Some(Token::Text(string)) => {
                    children.push(Node::Text(
                        str::from_utf8(string).expect("Invalid UTF-8").to_string(),
                    ));
                    self.advance();
                }
                Some(_) => {
                    let tag = self.parse_tag();
                    match tag {
                        Tag::Closing => break,
                        Tag::Opening(node) => children.push(node),
                    }
                }
                None => break,
            }
        }

        Tag::Opening(Node::Element {
            tag,
            attributes,
            children,
        })
    }

    #[inline]
    fn parse_attr(&mut self) -> String {
        let Token::Ident(_) = self.current.unwrap() else {
            unreachable!()
        };
        self.advance(); // Consume ident

        self.expect_and_consume(TokenKind::OpAssign);
        self.expect(TokenKind::String);

        let Token::String(data) = self.current.unwrap() else {
            panic!("Invalid assignment, expected string value ...")
        };

        str::from_utf8(data).expect("Invalid UTF-8").to_string()
    }
}
