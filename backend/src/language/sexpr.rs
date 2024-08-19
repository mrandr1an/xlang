use super::token::*;

#[derive(Debug)]
pub enum Sexpr<'a> {
    EndList,
    Item(Token<'a>),
    List {
        car: Box<Option<Sexpr<'a>>>,
        cdr: Vec<Sexpr<'a>>,
    },
}

impl<'a> Sexpr<'a> {
    pub fn new() -> Self {
        Self::List {
            car: Box::new(None),
            cdr: Vec::new(),
        }
    }

    pub fn add(&mut self, item: Sexpr<'a>) {
        if let Sexpr::List { car, cdr } = self {
            match **car {
                None => **car = Some(item),
                Some(Sexpr::EndList) => panic!("Cannot add more items to list"),
                Some(_) => {
                    if let Some(Sexpr::EndList) = cdr.last() {
                        panic!("List has ended, cannot add more items")
                    } else {
                        cdr.push(item)
                    }
                }
            }
        } else {
            panic!("Cannot add item to non list")
        }
    }

    pub fn endlist(&mut self) {
        if let Sexpr::List { car, cdr } = self {
            match **car {
                None => **car = Some(Sexpr::EndList),
                Some(_) => cdr.push(Sexpr::EndList),
            }
        } else {
            panic!("Cannot add item to non list")
        }
    }
}

#[derive(Debug)]
pub enum List<'a> {
    Void,
    Packed { car: Sexpr<'a>, cdr: Vec<Sexpr<'a>> },
}

impl<'a> From<Option<(Sexpr<'a>, Vec<Sexpr<'a>>)>> for List<'a> {
    fn from(value: Option<(Sexpr<'a>, Vec<Sexpr<'a>>)>) -> Self {
        match value {
            None => Self::Void,
            Some((head, tail)) => List::Packed {
                car: head,
                cdr: tail,
            },
        }
    }
}

impl<'a> List<'a> {}

impl<'a> Sexpr<'a> {
    pub fn next(self) -> Option<Result<Token<'a>, List<'a>>> {
        match self {
            Self::EndList => None,
            Self::Item(item) => Some(Ok(item)),
            Self::List { car, cdr } => match *car {
                Some(head) => Some(Err(List::from(Some((head, cdr))))),
                None => Some(Err(List::from(None))),
            },
        }
    }
}
