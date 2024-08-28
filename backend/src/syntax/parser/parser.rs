use crate::syntax::tokenizer::{
    token::Token,
    tokenized::{Tokenized, TokenizerError},
};

pub enum ParserError<'a> {
    Tokenizer(TokenizerError<'a>),
}

impl<'a> From<TokenizerError<'a>> for ParserError<'a> {
    fn from(value: TokenizerError<'a>) -> Self {
        Self::Tokenizer(value)
    }
}
