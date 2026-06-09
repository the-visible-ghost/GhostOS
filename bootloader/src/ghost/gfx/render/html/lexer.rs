use super::token::Token;
use alloc::fmt;

#[derive(Clone, Copy)]
enum LexerMode {
    Text,
    Tag,
}

struct LexerSnapshot {
    pub index: usize,
    pub current: u8,
    pub mode: LexerMode,
}

pub struct Lexer<'lexer> {
    index: usize,
    current: u8,
    html: &'lexer [u8],
    mode: LexerMode,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(html: &'lexer [u8]) -> Self {
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

    #[inline]
    fn advance(&mut self) {
        self.index += 1;
        match self.index >= self.html.len() {
            true => self.current = 0,
            false => self.current = self.html[self.index],
        }
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.current != 0
            && (self.current == b' ' || self.current == b'\t' || self.current == b'\n')
        {
            self.advance();
        }
    }

    #[inline]
    fn snapshot(&self) -> LexerSnapshot {
        LexerSnapshot {
            current: self.current,
            index: self.index,
            mode: self.mode,
        }
    }

    #[inline]
    fn load_snapshot(&mut self, snapshot: LexerSnapshot) {
        self.current = snapshot.current;
        self.index = snapshot.index;
        self.mode = snapshot.mode;
    }

    pub fn peek_token(&mut self) -> Option<Token<'lexer>> {
        let snapshot = self.snapshot();
        let token = self.next_token();
        self.load_snapshot(snapshot);
        token
    }

    pub fn next_token(&mut self) -> Option<Token<'lexer>> {
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

    #[inline]
    fn lex_ident(&mut self) -> Option<Token<'lexer>> {
        let start_pos = self.index;

        while (b'0' <= self.current && self.current <= b'9')
            || (b'A' <= self.current && self.current <= b'Z')
            || (b'a' <= self.current && self.current <= b'z')
            || self.current == b'_'
            || self.current == b'-'
        {
            self.advance();
        }

        if self.current == 0 {
            return None;
        }
        Some(Token::Ident(&self.html[start_pos..self.index]))
    }

    #[inline]
    fn lex_string(&mut self) -> Option<Token<'lexer>> {
        let started_with = self.current;
        self.advance(); // consume quote
        let start_pos = self.index;
        while self.current != started_with && self.current != 0 {
            self.advance();
        }
        if self.current != 0 {
            let token = Token::String(&self.html[start_pos..self.index]);
            self.advance(); // consume quote
            Some(token)
        } else {
            None
        }
    }

    #[inline]
    fn lex_text(&mut self) -> Option<Token<'lexer>> {
        let start_pos = self.index;
        while self.current != b'<' && self.current != 0 {
            self.advance();
        }
        match self.current {
            0 => None,
            _ => Some(Token::Text(&self.html[start_pos..self.index])),
        }
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
