use std::ops::Range;

#[derive(Debug)]
pub enum TokenType<'a> {
    L_PAREN,
    R_PAREN,
    END_LINE,
    STRING(SexprItem<'a>),
}

#[derive(Debug)]
pub struct Token<'a> {
    range: Range<usize>,
    pub token: TokenType<'a>,
}

impl<'a> From<&'a str> for TokenType<'a> {
    fn from(value: &'a str) -> Self {
        match value.chars().nth(0).unwrap() {
            '#' => TokenType::STRING(SexprItem::Extension(value)),
            ':' => TokenType::STRING(SexprItem::Symbol(value)),
            _ => {
                if value == "nil" {
                    TokenType::STRING(SexprItem::Nil)
                } else {
                    TokenType::STRING(SexprItem::Atom(value))
                }
            }
        }
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
pub enum SexprItem<'a> {
    Atom(&'a str),
    Symbol(&'a str),
    Extension(&'a str),
    Nil,
}
