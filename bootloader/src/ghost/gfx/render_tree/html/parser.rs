use super::lexer::Token;

pub struct Parser<'parser> {
    current: Token<'parser>,
}
