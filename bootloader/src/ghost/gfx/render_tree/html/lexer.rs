extern crate alloc;
use alloc::vec::Vec;

pub enum Token<'a> {
    AngleOpen,
    AngleClose,
    Indent(&'a [u8]),
    String(&'a [u8]),
    // Number(&'a str),
    OpAssign,
    Slash,
    EndOfFile,
}

#[derive(PartialEq)]
enum LexerMode {
    Text,
    Tag,
}

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

    pub fn next_token(&mut self) -> Token {
        if self.current != 0 {
            self.skip_whitespace();

            match self.mode {
                LexerMode::Tag => {
                    if (b'A' <= self.current && self.current <= b'Z')
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

            match self.current {
                b'<' => {
                    self.mode = LexerMode::Tag;
                    return Token::AngleOpen;
                }
                b'>' => {
                    self.mode = LexerMode::Text;
                    return Token::AngleClose;
                }
                b'/' => return Token::Slash,
                b'=' => return Token::OpAssign,
                _ => { /* ignore others */ }
            }
        }
        Token::EndOfFile
    }

    fn lex_ident(&mut self) -> Token<'_> {
        let start_pos = self.index;

        while (b'A' <= self.current && self.current <= b'Z')
            || (b'a' <= self.current && self.current <= b'z')
            || self.current == b'_'
            || self.current == b'-'
        {
            self.advance();
        }

        Token::Indent(&self.html[start_pos..self.index])
    }

    fn lex_string(&mut self) -> Token<'_> {
        let started_with = self.current;
        self.advance(); // consume quote
        let start_pos = self.index;
        while self.current == started_with {
            self.advance();
        }
        let token = Token::String(&self.html[start_pos..self.index]);
        self.advance(); // consume quote
        token
    }

    fn lex_text(&mut self) -> Token<'_> {
        todo!()
    }
}
