use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use crate::syntax::tokenizer::token::Lexeme;

#[derive(Debug)]
pub enum Sexpr<'a> {
    Item(Item<'a>),
    List(Rc<List<'a>>),
}

#[derive(Debug)]
pub struct Item<'a> {
    lexeme: Lexeme<'a>,
    parent: Weak<List<'a>>,
}

#[derive(Debug)]
pub struct List<'a> {
    parent: Option<Weak<List<'a>>>,
    elements: SexprElements<'a>,
}

impl<'a> List<'a> {
    pub fn root() -> Rc<Self> {
        Rc::new(List {
            parent: None,
            elements: RefCell::new(vec![]),
        })
    }

    pub fn new(parent: Weak<List<'a>>) -> Rc<Self> {
        Rc::new(List {
            parent: Some(parent),
            elements: RefCell::new(vec![]),
        })
    }

    pub fn get_parent(&self) -> Option<Rc<List<'a>>> {
        self.parent
            .as_ref()
            .map(|parent| parent.upgrade().expect("Weak pointer error"))
    }

    pub fn push_lexeme(self: &Rc<Self>, lexeme: Lexeme<'a>) {
        self.elements.borrow_mut().push(Sexpr::Item(Item {
            lexeme,
            parent: Rc::downgrade(self),
        }))
    }

    pub fn push_list(self: &Rc<Self>, list: Rc<List<'a>>) {
        self.elements.borrow_mut().push(Sexpr::List(list))
    }

    pub fn push_sexpr(self: &Rc<Self>, sexpr: Sexpr<'a>) {
        self.elements.borrow_mut().push(sexpr)
    }

    fn iter(self: &'a Rc<Self>) -> ListIter<'a> {
        let borrowed_self = self.elements.borrow();
        /* For future reference vector.as_slice() is == to &vector[..] latter is more elegant imo*/
        ListIter {
            inner: Some(Ref::map(borrowed_self, |inner_vector| &inner_vector[..])),
        }
    }
}

pub struct ListIter<'a> {
    inner: Option<Ref<'a, [Sexpr<'a>]>>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = Ref<'a, Sexpr<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(slice_ref) => match *slice_ref {
                [] => None,
                [_, ..] => {
                    let (head_ref, tail_ref) =
                        Ref::map_split(slice_ref, |slice| (&slice[0], &slice[1..]));
                    self.inner.replace(tail_ref);
                    Some(head_ref)
                }
            },
            None => None,
        }
    }
}

/*
http://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=517e738082574de914918476cb47c493
https://stackoverflow.com/questions/33541492/returning-iterator-of-a-vec-in-a-refcell
 */

type SexprElements<'a> = RefCell<Vec<Sexpr<'a>>>;

impl<'a> Sexpr<'a> {
    pub fn root() -> Self {
        Self::List(List::root())
    }

    pub fn new(parent: &Sexpr<'a>) -> Self {
        match parent {
            Sexpr::List(parent_list) => Self::List(List::new(Rc::downgrade(parent_list))),
            Sexpr::Item(_) => panic!("Item cannot be parent"),
        }
    }

    pub fn add_lexeme(&self, lexeme: Lexeme<'a>) {
        match self {
            Self::List(list) => list.push_lexeme(lexeme),
            Self::Item(_) => panic!("Cannot add lexeme to non list"),
        }
    }

    pub fn add_list(&self, list: Rc<List<'a>>) {
        match self {
            Self::List(curr_list) => curr_list.push_list(list),
            Self::Item(_) => panic!("Cannot add list to non list"),
        }
    }

    pub fn add_sexpr(&self, sexpr: &Sexpr<'a>) {
        match self {
            Self::List(curr_list) => match sexpr {
                Sexpr::Item(_) => todo!("Maybe this should not be implemented"),
                Sexpr::List(list) => curr_list.push_list(Rc::clone(list)),
            },
            Self::Item(_) => panic!("Cannot add sexpr to non list"),
        }
    }

    pub fn get_parent(&self) -> Option<Rc<List<'a>>> {
        match self {
            Sexpr::Item(item) => item.parent.upgrade(),
            Sexpr::List(list) => {
                if let Some(par) = &list.parent {
                    par.upgrade()
                } else {
                    None
                }
            }
        }
    }

    pub fn iter(&'a self) -> SexprIter<'a> {
        match self {
            Sexpr::List(list) => SexprIter {
                list_iter: list.iter(),
            },
            Sexpr::Item(_) => panic!("Cannot iterate an item"),
        }
    }
}

pub struct SexprIter<'a> {
    list_iter: ListIter<'a>,
}

impl<'a> Iterator for SexprIter<'a> {
    type Item = Ref<'a, Sexpr<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.list_iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::tokenizer::token::Lexeme;

    use super::Sexpr;

    #[test]
    fn creating_sexprs() {
        let root = Sexpr::root();

        /* Pseudo lexemes */
        let defun = Lexeme::Symbol {
            range: 0..5,
            literal: "defun",
        };
        let print = Lexeme::Symbol {
            range: 0..5,
            literal: "print",
        };

        root.add_lexeme(defun);
        root.add_lexeme(print);

        let args = Sexpr::new(&root);

        let arg1 = Lexeme::Atom {
            range: 10..15,
            literal: "string",
        };
        let arg2 = Lexeme::Symbol {
            range: 10..15,
            literal: "msg",
        };
        args.add_lexeme(arg1);
        args.add_lexeme(arg2);

        root.add_sexpr(&args);

        println!("{:#?}", root);
    }

    #[test]
    fn iterating_sexprs() {
        let root = Sexpr::root();

        /* Pseudo lexemes */
        let defun = Lexeme::Symbol {
            range: 0..5,
            literal: "defun",
        };
        let print = Lexeme::Symbol {
            range: 0..5,
            literal: "print",
        };

        root.add_lexeme(defun);
        root.add_lexeme(print);

        let args = Sexpr::new(&root);

        let arg1 = Lexeme::Atom {
            range: 10..15,
            literal: "string",
        };
        let arg2 = Lexeme::Symbol {
            range: 10..15,
            literal: "msg",
        };
        args.add_lexeme(arg1);
        args.add_lexeme(arg2);

        root.add_sexpr(&args);

        for i in root.iter() {
            println!("{:#?}", i)
        }
    }
}
