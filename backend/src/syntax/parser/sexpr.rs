use std::{iter::Peekable, vec::IntoIter};

use crate::syntax::tokenizer::{
    token::{Lexeme, Token},
    tokenized::{Tokenized, TokenizerError},
};

#[derive(Debug)]
pub struct List<'a> {
    items: Vec<SexprType<'a>>,
    parent: Option<ListPtr>,
}

impl<'a> List<'a> {
    fn root() -> Self {
        Self {
            items: Vec::new(),
            parent: None,
        }
    }

    fn new(parent: ListPtr) -> Self {
        Self {
            items: Vec::new(),
            parent: None,
        }
    }

    fn iter(self) -> IntoIter<SexprType<'a>> {
        self.items.into_iter()
    }

    fn get_parent(&self) -> Option<ListPtr> {
        self.parent
    }
}

#[derive(Debug)]
pub struct Item<'a> {
    data: Lexeme<'a>,
    parent: ListPtr,
}

impl<'a> Item<'a> {
    fn new(lexeme: Lexeme<'a>, parent: ListPtr) -> Self {
        Self {
            data: lexeme,
            parent,
        }
    }
}

type ListPtr = usize;

#[derive(Debug)]
pub enum SexprType<'a> {
    List(ListPtr),
    Item(Item<'a>),
}

#[derive(Debug)]
pub struct Arena<'a> {
    lists: Vec<List<'a>>,
}

impl<'a> Arena<'a> {
    fn new() -> Self {
        Self { lists: Vec::new() }
    }

    fn push(&mut self, list: List<'a>) -> ListPtr {
        self.lists.push(list);
        self.lists.len() - 1
    }

    fn get(&self, id: ListPtr) -> Option<&List<'a>> {
        self.lists.get(id)
    }

    fn get_mut(&mut self, id: ListPtr) -> Option<&mut List<'a>> {
        self.lists.get_mut(id)
    }
}

#[derive(Debug)]
pub struct Zipper<'a> {
    mem: Arena<'a>,
}

#[derive(Debug)]
pub enum ZipperError<'a> {
    TokenizerError(TokenizerError<'a>),
    UnexpectedEOF,
}

impl<'a> From<TokenizerError<'a>> for ZipperError<'a> {
    fn from(value: TokenizerError<'a>) -> Self {
        ZipperError::TokenizerError(value)
    }
}

//Zipper creation from tokens
impl<'a> Zipper<'a> {
    fn new(tokens: &'a mut Tokenized<'a>) -> Option<Result<Self, Vec<ZipperError<'a>>>> {
        let (tokens, errors): (Vec<_>, Vec<_>) = tokens.partition(Result::is_ok);
        if errors.is_empty() {
            let mut tokens = tokens
                .into_iter()
                .map(Result::unwrap)
                .collect::<Vec<_>>()
                .into_iter()
                .peekable();
            // let zippers = Vec::new();
            match Self::next_root(&mut tokens) {
                Some(Ok(zipper)) => Some(Ok(zipper)),
                Some(Err(err)) => Some(Err(vec![err])),
                None => None,
            }
        } else {
            let errors = errors
                .into_iter()
                .map(|error| ZipperError::from(error.unwrap_err()))
                .collect::<Vec<_>>();
            Some(Err(errors))
        }
    }

    fn next_root(
        tokens: &mut Peekable<IntoIter<Token<'a>>>,
    ) -> Option<Result<Self, ZipperError<'a>>> {
        match tokens.next() {
            Some(Token::LParen(pos)) => {
                let mut arena = Arena::new();
                let root = List::root();
                match Self::next_sexpr(tokens, arena.push(root), None, &mut arena) {
                    Ok(_) => Some(Ok(Self { mem: arena })),
                    Err(err) => Some(Err(err)),
                }
            }
            Some(Token::RParen(pos)) => todo!(), //expected lparen
            Some(Token::Lexeme(lexeme)) => todo!(), //expected lparen
            None => None,                        //expected lparen
        }
    }

    fn next_sexpr(
        tokens: &mut Peekable<IntoIter<Token<'a>>>,
        cur: ListPtr,
        parent: Option<ListPtr>,
        mem: &mut Arena<'a>,
    ) -> Result<(), ZipperError<'a>> {
        match tokens.next() {
            Some(Token::LParen(pos)) => {
                //Create new list that points to its parent in arena
                let sexpr = List::new(cur);
                Self::next_sexpr(tokens, mem.push(sexpr), Some(cur), mem)
            }
            Some(Token::RParen(pos)) => match parent {
                Some(par_of_cur_ptr) => {
                    let par_of_cur = mem
                        .get_mut(par_of_cur_ptr)
                        .expect("Parent not found (shouldnt happen)");
                    par_of_cur.items.push(SexprType::List(cur));
                    match par_of_cur.get_parent() {
                        Some(par_of_par_ptr) => {
                            Self::next_sexpr(tokens, par_of_cur_ptr, Some(par_of_par_ptr), mem)
                        }
                        None => Self::next_sexpr(tokens, par_of_cur_ptr, None, mem),
                    }
                }
                None => Ok(()),
            }, //expected lparen
            Some(Token::Lexeme(lexeme)) => {
                let cur_list = mem
                    .get_mut(cur)
                    .expect("Current not found (shouldnt hapen)");
                cur_list.items.push(SexprType::Item(Item::new(lexeme, cur)));
                Self::next_sexpr(tokens, cur, parent, mem)
            } //expected lparen
            None => Err(ZipperError::UnexpectedEOF), //expected lparen
        }
    }

    pub fn iter(self) -> Sexpr<'a> {
        Sexpr { mem: self.mem }
    }
}

pub struct Sexpr<'a> {
    mem: Arena<'a>,
}

impl<'a> Sexpr<'a> {
    fn traverse_ref(&'a self) -> InorderTraverseRef<'a> {
        InorderTraverseRef {
            next: (0, 0),
            cur_node: None,
            mem_ref: &self.mem,
        }
    }
}

pub struct InorderTraverseRef<'a> {
    next: (ListPtr, usize),
    cur_node: Option<&'a List<'a>>,
    mem_ref: &'a Arena<'a>,
}

impl<'a> Traverser for InorderTraverseRef<'a> {
    type Item = &'a List<'a>;
    fn step(&mut self) -> Option<Self::Item> {
        let list = self.mem_ref.get(self.next.0);
        self.next.0 += 1;
        list
    }
}

pub trait Traverser {
    type Item;
    fn step(&mut self) -> Option<Self::Item>;
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        parser::sexpr::{Traverser, Zipper},
        tokenizer::tokenized::Tokens,
    };

    #[test]
    fn parse() {
        let mut s = "(defun print ( :string msg ) \"Prints msg\" ( dbg msg ) ) ( exit 1 )".tokens();
        let zipper = Zipper::new(&mut s).unwrap().unwrap();
        let sexpr = zipper.iter();
        let mut traverser = sexpr.traverse_ref();

        println!("{:#?}", traverser.step());
        println!("{:#?}", traverser.step());
        println!("{:#?}", traverser.step());
    }
}
