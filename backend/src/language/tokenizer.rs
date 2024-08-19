use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::Token;

#[derive(Debug)]
pub struct Tokenizer<'a, 'b>
where
    'b: 'a,
{
    program: Peekable<Enumerate<Chars<'a>>>,
    program_source: &'b str,
}

impl<'a, 'b> Tokenizer<'a, 'b>
where
    'b: 'a,
{
    fn next_skip_whitespace(&mut self, start: usize) -> Option<Token<'b>> {
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

    pub fn new(source: &'b String) -> Self {
        Tokenizer {
            program_source: source,
            program: source.chars().enumerate().peekable(),
        }
    }
}

impl<'a, 'b> Iterator for Tokenizer<'a, 'b>
where
    'b: 'a,
{
    type Item = Token<'b>;
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
