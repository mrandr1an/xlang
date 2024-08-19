use super::token::{Token, TokenType};

#[derive(Debug)]
pub enum SyntaxError<'a> {
    Expected {
        expected: TokenType<'a>,
        got: Token<'a>,
    },
    UnexpectedEnd,
    UnexpectedStart,
}

impl<'a> SyntaxError<'a> {}
