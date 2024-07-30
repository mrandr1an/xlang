use super::sexpr::Sexpr;

pub enum AST<'a> {
    Module(Vec<Sexpr<'a>>),
    Sexpr(Sexpr<'a>),
}
