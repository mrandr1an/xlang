use super::{error::SyntaxError, sexpr::Sexpr, token::TokenType, tokenizer::Tokenizer};

pub struct Parser<'a, 'b>
where
    'b: 'a,
{
    iter: Tokenizer<'a, 'b>,
}

impl<'a, 'b> From<Tokenizer<'a, 'b>> for Parser<'a, 'b> {
    fn from(value: Tokenizer<'a, 'b>) -> Self {
        Self { iter: value }
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse(mut self) -> Option<(Result<Sexpr<'a>, SyntaxError<'a>>, Parser<'a, 'b>)> {
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
