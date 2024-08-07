use super::sexpr::Sexpr;

pub enum AST<'a> {
    Module {
        depends: Option<Vec<AST<'a>>>,
        code: Vec<Sexpr<'a>>,
    },
    Empty,
}

impl<'a> AST<'a> {}
