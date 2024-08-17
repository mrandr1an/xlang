use super::sexpr::Sexpr;

pub enum AST<'a, T> {
    Depends { on: &'a AST<'a, T>, program_type: T },
    Standalone { program_type: T },
}
