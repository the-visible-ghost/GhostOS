#[derive(PartialEq, Debug)]
pub enum TokenKind {
    AngleOpen,
    AngleClose,
    Ident,
    String,
    Text,
    OpAssign,
    Slash,
}

#[derive(Clone, Copy)]
pub enum Token<'token> {
    AngleOpen,
    AngleClose,
    Ident(&'token [u8]),
    String(&'token [u8]),
    Text(&'token [u8]),
    OpAssign,
    Slash,
}

impl<'token> Token<'token> {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::AngleOpen => TokenKind::AngleOpen,
            Token::AngleClose => TokenKind::AngleClose,
            Token::Ident(_) => TokenKind::Ident,
            Token::String(_) => TokenKind::String,
            Token::Text(_) => TokenKind::Text,
            Token::OpAssign => TokenKind::OpAssign,
            Token::Slash => TokenKind::Slash,
        }
    }
}
