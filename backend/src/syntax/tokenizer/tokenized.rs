use std::{iter::Enumerate, str::Chars};

use super::token::{Lexeme, Token};

#[derive(Debug)]
pub enum TokenizerError<'a> {
    StringDelimeterError {
        start: usize,
        end: usize,
        literal: &'a str,
    },
    EndWithAtom {
        start: usize,
        end: usize,
        literal: &'a str,
    },
    EndWithExt {
        start: usize,
        end: usize,
        literal: &'a str,
    },
    EndWithSymbol {
        start: usize,
        end: usize,
        literal: &'a str,
    },
}

#[derive(Debug)]
pub struct Tokenized<'a> {
    chars: Enumerate<Chars<'a>>,
    input: &'a str,
}

impl<'a> Tokenized<'a> {
    fn tokenize_string(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Result<Token<'a>, TokenizerError<'a>>> {
        match self.chars.next() {
            Some((_, '"')) => Some(Ok(Token::Lexeme(Lexeme::String {
                range: start + 1..end,
                literal: &self.input[start + 1..end + 1],
            }))),
            None => Some(Err(TokenizerError::StringDelimeterError {
                start,
                end,
                literal: &self.input[start..end],
            })),
            Some((offset, _)) => self.tokenize_string(start, offset),
        }
    }

    fn tokenize_atom(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Result<Token<'a>, TokenizerError<'a>>> {
        match self.chars.next() {
            Some((_, ' ')) => Some(Ok(Token::Lexeme(Lexeme::Atom {
                range: start + 1..end,
                literal: &self.input[start + 1..end + 1],
            }))),
            Some((offset, _)) => self.tokenize_atom(start, offset),
            None => Some(Err(TokenizerError::EndWithAtom {
                start,
                end,
                literal: &self.input[start..end],
            })),
        }
    }

    fn tokenize_symbol_or_nil(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Result<Token<'a>, TokenizerError<'a>>> {
        match self.chars.next() {
            Some((_, ' ')) => {
                if &self.input[start..end + 1] == "nil" {
                    Some(Ok(Token::Lexeme(Lexeme::Nil { range: start..end })))
                } else {
                    Some(Ok(Token::Lexeme(Lexeme::Symbol {
                        range: start..end,
                        literal: &self.input[start..end + 1],
                    })))
                }
            }
            Some((offset, _)) => self.tokenize_symbol_or_nil(start, offset),
            None => Some(Err(TokenizerError::EndWithSymbol {
                start,
                end,
                literal: &self.input[start..end],
            })),
        }
    }

    fn tokenize_extension(
        &mut self,
        start: usize,
        end: usize,
    ) -> Option<Result<Token<'a>, TokenizerError<'a>>> {
        match self.chars.next() {
            Some((_, ' ')) => Some(Ok(Token::Lexeme(Lexeme::Extension {
                range: start + 1..end,
                literal: &self.input[start + 1..end + 1],
            }))),
            Some((offset, _)) => self.tokenize_extension(start, offset),
            None => Some(Err(TokenizerError::EndWithExt {
                start,
                end,
                literal: &self.input[start..end],
            })),
        }
    }

    // fn parse(self) -> Parsed<'a> {
    //     Parsed::new(self)
    // }
}

impl<'a> Iterator for Tokenized<'a> {
    type Item = Result<Token<'a>, TokenizerError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(character) => match character {
                (offset, '(') => Some(Ok(Token::LParen(offset))),
                (offset, ')') => Some(Ok(Token::RParen(offset))),
                (offset, other) => match other {
                    '#' => self.tokenize_extension(offset, offset),
                    ':' => self.tokenize_atom(offset, offset),
                    '"' => self.tokenize_string(offset, offset),
                    ' ' => self.next(),
                    _ => self.tokenize_symbol_or_nil(offset, offset),
                },
            },
            None => None,
        }
    }
}

pub trait Tokens<'a> {
    fn tokens(&'a self) -> Tokenized<'a>;
}

impl<'a, T: AsRef<str>> Tokens<'a> for T {
    fn tokens(&'a self) -> Tokenized<'a> {
        Tokenized {
            chars: self.as_ref().chars().enumerate(),
            input: self.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tokens;

    #[test]
    fn test_simple_tokens() {
        let input = "(This #tokenizable :wow expression nil \"string test\")";
        for i in input.tokens() {
            println!("{:#?}", i)
        }
    }

    #[test]
    fn test_complex_tokens() {
        let input = "(defun print (:string msg) \"Prints msg\" (dbg msg) ) ";
        for i in input.tokens() {
            println!("{:#?}", i)
        }
    }
}
