use std::ops::Range;

#[derive(Debug)]
pub enum TokenType<'a> {
    L_PAREN,
    R_PAREN,
    END_LINE,
    STRING(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    range: Range<usize>,
    token: TokenType<'a>,
}

impl<'a> From<&'a str> for TokenType<'a> {
    fn from(value: &'a str) -> Self {
        TokenType::STRING(value)
    }
}

impl<'a> From<char> for TokenType<'a> {
    fn from(value: char) -> Self {
        match value {
            '(' => TokenType::L_PAREN,
            ')' => TokenType::R_PAREN,
            '\n' => TokenType::END_LINE,
            c => panic!("Bad interaction fix later"),
        }
    }
}

impl<'a> From<(usize, char)> for Token<'a> {
    fn from(value: (usize, char)) -> Self {
        Token {
            range: value.0..value.0,
            token: TokenType::from(value.1),
        }
    }
}

impl<'a> From<(Range<usize>, &'a str)> for Token<'a> {
    fn from(value: (Range<usize>, &'a str)) -> Self {
        Token {
            range: value.0,
            token: TokenType::from(value.1),
        }
    }
}

#[derive(Debug)]
pub enum SexprItemType {
    Atom,
    Symbol,
    Extension,
    Nil,
}

#[derive(Debug)]
pub struct SexprItem<'a> {
    range: Range<usize>,
    item: SexprItemType,
    literal: &'a str,
}
