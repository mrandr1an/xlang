use std::rc::Rc;

use crate::syntax::tokenizer::{
    token::{Lexeme, Token},
    tokenized::{Tokenized, TokenizerError},
};

use super::sexpr::{List, Sexpr};

#[derive(Debug)]
pub enum ParserError<'a> {
    Tokenizer(TokenizerError<'a>),
    UnexpectedR(usize),
    UnexpectedLexeme(Lexeme<'a>),
    UnexpectedEOF,
}

impl<'a> From<TokenizerError<'a>> for ParserError<'a> {
    fn from(value: TokenizerError<'a>) -> Self {
        Self::Tokenizer(value)
    }
}

#[derive(Debug)]
pub struct Parsed<'a> {
    tokens: Tokenized<'a>,
    roots: Vec<Sexpr<'a>>,
}

impl<'a, 'b: 'a> Parsed<'b> {
    fn new(tokens: Tokenized<'b>) -> Self {
        Self {
            tokens,
            roots: Vec::new(),
        }
    }

    fn parse_list(
        &'a mut self,
        cur: Rc<List<'a>>,
        parent: Option<Rc<List<'a>>>,
    ) -> Result<Rc<List<'a>>, ParserError<'a>> {
        match self.tokens.next() {
            Some(Ok(Token::LParen(pos))) => {
                // println!("lparen");
                let new_list = List::new(Rc::downgrade(&cur));
                match parent {
                    Some(_) => self.parse_list(new_list, Some(cur)),
                    None => self.parse_list(new_list, Some(cur)),
                }
            }
            Some(Ok(Token::RParen(pos))) => {
                // println!("rparen");
                match parent {
                    Some(par_of_cur) => {
                        par_of_cur.push_list(cur);
                        match par_of_cur.get_parent() {
                            Some(par_of_par) => self.parse_list(par_of_cur, Some(par_of_par)),
                            None => self.parse_list(par_of_cur, None),
                        }
                    }
                    None => Ok(cur),
                }
            }
            Some(Ok(Token::Lexeme(lexeme))) => {
                // println!("lexeme: {:#?}", lexeme);
                cur.push_lexeme(lexeme);
                self.parse_list(cur, parent)
            }
            Some(Err(err)) => Err(ParserError::from(err)),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    fn next(&'a mut self) -> Option<Result<Sexpr<'a>, ParserError<'a>>> {
        match self.tokens.next() {
            Some(Ok(Token::LParen(pos))) => {
                let root = List::root();
                match self.parse_list(root, None) {
                    Ok(list) => Some(Ok(Sexpr::List(list))),
                    Err(err) => Some(Err(err)),
                }
            }
            Some(Ok(Token::RParen(pos))) => Some(Err(ParserError::UnexpectedR(pos))),
            Some(Ok(Token::Lexeme(lexeme))) => Some(Err(ParserError::UnexpectedLexeme(lexeme))),
            Some(Err(err)) => Some(Err(ParserError::from(err))),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::tokenized::Tokens;

    use super::Parsed;

    #[test]
    fn parse() {
        let s = "(defun print ( :string msg ) \"Prints msg\" ( dbg msg ) ) ( exit 1 )".tokens();
        let mut parsed = Parsed::new(s);
        println!("first {:#?}", parsed.next());
        println!("second {:#?}", parsed.next());
        println!("{:#?}", parsed.next());
    }
}
