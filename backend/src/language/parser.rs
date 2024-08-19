use super::{error::SyntaxError, sexpr::Sexpr, token::TokenType, tokenizer::Tokenizer};

pub struct Parser<'a> {
    iter: Tokenizer<'a>,
}

impl<'a> From<Tokenizer<'a>> for Parser<'a> {
    fn from(value: Tokenizer<'a>) -> Self {
        Self { iter: value }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&'a mut self) -> Option<(Result<Sexpr<'a>, SyntaxError<'a>>, &'a mut Parser<'a>)> {
        let root = Sexpr::new();
        match self.iter.next() {
            Some(token) => match token.token {
                TokenType::L_PAREN => Some((self.next(root), self)),
                _ => Some((Err(SyntaxError::UnexpectedStart), self)),
            },
            None => None,
        }
    }

    fn new_list(&mut self) -> Result<Sexpr<'a>, SyntaxError<'a>> {
        let root = Sexpr::new();
        self.next(root)
    }

    fn next(&mut self, mut parent: Sexpr<'a>) -> Result<Sexpr<'a>, SyntaxError<'a>> {
        match self.iter.next() {
            Some(token) => match token.token {
                TokenType::L_PAREN => match self.new_list() {
                    Ok(list) => {
                        parent.add(list);
                        self.next(parent)
                    }
                    Err(err) => Err(err),
                },
                TokenType::R_PAREN => {
                    parent.endlist();
                    Ok(parent)
                }
                TokenType::END_LINE => todo!(),
                TokenType::STRING(_) => {
                    parent.add(Sexpr::Item(token));
                    self.next(parent)
                }
            },
            None => Err(SyntaxError::UnexpectedEnd),
        }
    }
}
