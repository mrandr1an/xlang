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

impl<'a> Iterator for Sexpr<'a> {
    type Item = Sexpr<'a>;
    fn next(&mut self) -> Option<Sexpr<'a>> {
        todo!()
    }
}
