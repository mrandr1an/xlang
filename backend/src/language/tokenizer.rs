use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::Token;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    program: Peekable<Enumerate<Chars<'a>>>,
    program_source: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn next_skip_whitespace(&mut self, start: usize) -> Option<Token<'a>> {
        match self.program.next() {
            Some((index, ' ')) => self.next_skip_whitespace(index + 1),
            Some((index, '\n')) => Some(Token::from((index, '\n'))),
            Some((index, '(')) => Some(Token::from((index, '('))),
            Some((index, ')')) => Some(Token::from((index, ')'))),
            Some((_, _)) => match self.program.peek() {
                Some((index, ' ')) => Some(Token::from((
                    start..*index - 1,
                    &self.program_source[start..*index],
                ))),
                Some((index, '(')) => Some(Token::from((
                    start..*index - 1,
                    &self.program_source[start..*index],
                ))),
                Some((index, ')')) => Some(Token::from((
                    start..*index - 1,
                    &self.program_source[start..*index],
                ))),
                Some((_, _character)) => self.next_skip_whitespace(start),
                None => None,
            },
            None => None,
        }
    }

    pub fn new(source: &'a String) -> Self {
        Tokenizer {
            program_source: source,
            program: source.chars().enumerate().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.program.peek() {
            Some((offset, _)) => {
                let token_start = *offset;
                self.next_skip_whitespace(token_start)
            }
            None => None,
        }
    }
}
