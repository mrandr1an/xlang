use super::{assembler::assembly::Assembly, cached::Cached, saved::Saved};

pub enum Written<'a, A: Assembly<'a>> {
    Cached(Cached<'a, A>),
    Saved(Saved<'a, A>),
}

pub struct Compiled {}
