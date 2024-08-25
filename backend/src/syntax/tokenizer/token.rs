use std::ops::Range;

#[derive(Debug)]
pub enum Token<'a> {
    LParen(usize),
    RParen(usize),
    Lexeme(Lexeme<'a>),
}

#[derive(Debug)]
pub enum Lexeme<'a> {
    String {
        range: Range<usize>,
        literal: &'a str,
    },
    Atom {
        range: Range<usize>,
        literal: &'a str,
    },
    Symbol {
        range: Range<usize>,
        literal: &'a str,
    },
    Extension {
        range: Range<usize>,
        literal: &'a str,
    },
    Nil {
        range: Range<usize>,
    },
}
