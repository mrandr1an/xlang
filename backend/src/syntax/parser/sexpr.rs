use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    rc::{Rc, Weak},
    slice::{Iter, IterMut},
};

use crate::syntax::tokenizer::token::Lexeme;

#[derive(Debug)]
pub enum Sexpr<'a> {
    Item {
        lexeme: Lexeme<'a>,
        parent: Weak<Sexpr<'a>>,
    },
    List {
        elements: SexprElements<'a>,
        parent: Option<Weak<Sexpr<'a>>>,
    },
}

type SexprElements<'a> = RefCell<Vec<Rc<Sexpr<'a>>>>;

impl<'a> Sexpr<'a> {
    fn root() -> Rc<Self> {
        Rc::new(Self::List {
            elements: RefCell::new(vec![]),
            parent: None,
        })
    }

    fn new(parent: Weak<Sexpr<'a>>) -> Rc<Self> {
        Rc::new(Self::List {
            elements: RefCell::new(vec![]),
            parent: Some(parent),
        })
    }

    fn add_lexeme(self: Rc<Self>, lexeme: Lexeme<'a>) {
        match &*self {
            Sexpr::List { elements, parent } => elements.borrow_mut().push(Rc::new(Sexpr::Item {
                lexeme,
                parent: Rc::downgrade(&self),
            })),
            _ => panic!("Cannot add item to non list"),
        }
    }

    fn add_list(self: Rc<Self>, list: Rc<Sexpr<'a>>) {
        match &*self {
            Sexpr::List { elements, parent } => elements.borrow_mut().push(list),
            _ => panic!("Cannot add item to non list"),
        }
    }

    fn list_iter(list: &'a SexprElements<'a>) -> SexprIter<'a> {
        SexprIter {
            inner: Some(Ref::map(list.borrow(), |v| &v[..])),
        }
    }
}

/*
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=517e738082574de914918476cb47c493
https://stackoverflow.com/questions/33541492/returning-iterator-of-a-vec-in-a-refcell
*/
struct SexprIter<'a> {
    inner: Option<Ref<'a, [Rc<Sexpr<'a>>]>>,
}

impl<'a> Iterator for SexprIter<'a> {
    type Item = Ref<'a, Rc<Sexpr<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(borrow) => match *borrow {
                [] => None,
                [_, ..] => {
                    let (head, tail) = Ref::map_split(borrow, |slice| (&slice[0], &slice[1..]));
                    self.inner.replace(tail);
                    Some(head)
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::syntax::tokenizer::token::Lexeme;

    use super::Sexpr;

    #[test]
    fn create_sexpr() {
        let lexeme1 = Lexeme::Symbol {
            range: 1..5,
            literal: "defun",
        };

        let lexeme2 = Lexeme::Symbol {
            range: 7..11,
            literal: "print",
        };

        let root = Sexpr::root();
        Rc::clone(&root).add_lexeme(lexeme1);
        Rc::clone(&root).add_lexeme(lexeme2);

        let args = Sexpr::new(Rc::downgrade(&root));

        let arg1 = Lexeme::Symbol {
            range: 15..17,
            literal: "msg",
        };

        let arg2 = Lexeme::Atom {
            range: 18..23,
            literal: "string",
        };

        Rc::clone(&args).add_lexeme(arg1);
        Rc::clone(&args).add_lexeme(arg2);
        Rc::clone(&root).add_list(args);

        match &*root {
            Sexpr::List { elements, parent } => {
                let mut a = Sexpr::list_iter(elements);
                println!("first {:#?}", a.next());
                println!("second {:#?}", a.next());
                println!("third {:#?}", a.next());
                println!("fourth {:#?}", a.next());
            }
            _ => todo!(),
        }
    }
}
