use crate::syntax::tokenizer::{
    token::Token,
    tokenized::{Tokenized, TokenizerError, Tokens},
};

use super::sexpr::{Sexpr, SexprArena};

pub enum ParserError<'a> {
    TokenizerFailed(TokenizerError<'a>),
}

impl<'a> From<TokenizerError<'a>> for ParserError<'a> {
    fn from(value: TokenizerError<'a>) -> Self {
        Self::TokenizerFailed(value)
    }
}

pub struct Parsed<'a> {
    tokens: Tokenized<'a>,
    sexprs: SexprArena<'a>,
}

impl<'a> Parsed<'a> {
    pub fn new(tokens: Tokenized<'a>) -> Self {
        Self {
            tokens,
            sexprs: SexprArena::new(),
        }
    }
}

pub trait Expr<'a>: Tokens<'a> {
    fn exprs(&self) -> SexprArena<'a>;
}
