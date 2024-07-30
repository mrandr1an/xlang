use super::token::*;

#[derive(Debug)]
pub enum Sexpr<'a> {
    EndList,
    Item(SexprItem<'a>),
    List {
        car: Box<Sexpr<'a>>,
        cdr: Vec<Sexpr<'a>>,
    },
}
