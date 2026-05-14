extern crate alloc;
use alloc::fmt;

#[allow(dead_code)]
pub enum Token<'a> {
    AngleOpen,
    AngleClose,
    Ident(&'a [u8]),
    String(&'a [u8]),
    Text(&'a [u8]),
    // Number(&'a str),
    OpAssign,
    Slash,
}

#[derive(PartialEq, Debug)]
enum LexerMode {
    Text,
    Tag,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    index: usize,
    current: u8,
    html: &'a [u8],
    mode: LexerMode,
}

impl<'a> Lexer<'a> {
    pub fn new(html: &'a [u8]) -> Self {
        let mut lexer = Self {
            index: 0,
            current: 0,
            html,
            mode: LexerMode::Text,
        };
        if !html.is_empty() {
            lexer.current = html[0];
            lexer.mode = match lexer.current {
                b'<' => LexerMode::Tag,
                _ => LexerMode::Text,
            };
        }
        lexer
    }

    fn advance(&mut self) {
        self.index += 1;
        match self.index >= self.html.len() {
            true => self.current = 0,
            false => self.current = self.html[self.index],
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current != 0
            && (self.current == b' ' || self.current == b'\t' || self.current == b'\n')
        {
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        if self.current != 0 {
            self.skip_whitespace();

            match self.current {
                b'<' => {
                    self.mode = LexerMode::Tag;
                    self.advance();
                    return Some(Token::AngleOpen);
                }
                b'>' => {
                    self.mode = LexerMode::Text;
                    self.advance();
                    return Some(Token::AngleClose);
                }
                b'/' => {
                    self.advance();
                    return Some(Token::Slash);
                }
                b'=' => {
                    self.advance();
                    return Some(Token::OpAssign);
                }
                _ => {}
            }

            match self.mode {
                LexerMode::Tag => {
                    if (b'0' <= self.current && self.current <= b'9')
                        || (b'A' <= self.current && self.current <= b'Z')
                        || (b'a' <= self.current && self.current <= b'z')
                        || self.current == b'_'
                        || self.current == b'-'
                    {
                        return self.lex_ident();
                    } else if self.current == b'\"' || self.current == b'\'' {
                        return self.lex_string();
                    }
                }
                LexerMode::Text => {
                    return self.lex_text();
                }
            }
        }
        None
    }

    fn lex_ident(&mut self) -> Option<Token<'a>> {
        let start_pos = self.index;

        while (b'0' <= self.current && self.current <= b'9')
            || (b'A' <= self.current && self.current <= b'Z')
            || (b'a' <= self.current && self.current <= b'z')
            || self.current == b'_'
            || self.current == b'-'
        {
            self.advance();
        }

        Some(Token::Ident(&self.html[start_pos..self.index]))
    }

    fn lex_string(&mut self) -> Option<Token<'a>> {
        let started_with = self.current;
        self.advance(); // consume quote
        let start_pos = self.index;
        while self.current != started_with {
            self.advance();
        }
        let token = Token::String(&self.html[start_pos..self.index]);
        self.advance(); // consume quote
        Some(token)
    }

    fn lex_text(&mut self) -> Option<Token<'a>> {
        let start_pos = self.index;
        while self.current != b'<' {
            self.advance();
        }
        Some(Token::Text(&self.html[start_pos..self.index]))
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::AngleOpen => write!(f, "AngleOpen"),
            Token::AngleClose => write!(f, "AngleClose"),

            Token::Ident(bytes) => {
                write!(
                    f,
                    "Ident({})",
                    core::str::from_utf8(bytes).unwrap_or("<invalid utf8>")
                )
            }

            Token::String(bytes) => {
                write!(
                    f,
                    "String(\"{}\")",
                    core::str::from_utf8(bytes).unwrap_or("<invalid utf8>")
                )
            }

            Token::Text(bytes) => {
                write!(
                    f,
                    "Text({})",
                    core::str::from_utf8(bytes).unwrap_or("<invalid utf8>")
                )
            }

            Token::OpAssign => write!(f, "OpAssign"),
            Token::Slash => write!(f, "Slash"),
        }
    }
}
