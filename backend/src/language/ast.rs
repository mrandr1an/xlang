use super::sexpr::Sexpr;

pub enum AST<'a, T> {
    Depends { on: &'a AST<'a, T>, program_type: T },
    Standalone { program_type: T },
}

pub struct Depends {}

pub struct Standalone {}

pub struct Library {}

pub struct Executable {}
