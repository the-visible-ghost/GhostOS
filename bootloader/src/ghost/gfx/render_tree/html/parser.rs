use cssparser::Token;

pub struct Parser<'parser> {
    current: Token<'parser>,
}
