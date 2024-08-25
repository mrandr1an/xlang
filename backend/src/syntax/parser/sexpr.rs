use crate::syntax::tokenizer::token::Lexeme;

pub struct SexprArena<'a> {
    sexprs: Vec<Sexpr<'a>>,
}

impl<'a> SexprArena<'a> {
    pub fn new() -> Self {
        Self { sexprs: Vec::new() }
    }

    pub fn add_sexpr(&mut self, sexpr: Sexpr<'a>) {
        self.sexprs.push(sexpr)
    }

    pub fn len(&self) -> usize {
        self.sexprs.len()
    }
}

pub struct SexprId {
    index: usize,
}

pub struct ChildId {
    index: usize,
}

type Child<'a> = Result<Lexeme<'a>, Sexpr<'a>>;
type Children<'a> = Vec<Child<'a>>;

pub struct Sexpr<'a> {
    parent: Option<SexprId>,
    next_sibling: Option<SexprId>,
    prev_sibling: Option<SexprId>,
    first_child: Option<ChildId>,
    last_child: Option<ChildId>,

    data: Children<'a>,
}

impl<'a> Sexpr<'a> {
    pub fn new(
        parent: Option<SexprId>,
        next_sibling: Option<SexprId>,
        prev_sibling: Option<SexprId>,
        first_child: Option<ChildId>,
        last_child: Option<ChildId>,

        data: Children<'a>,
    ) -> Self {
        Self {
            parent,
            next_sibling,
            prev_sibling,
            first_child,
            last_child,
            data,
        }
    }

    pub fn add_child(&mut self, child: Child<'a>) {
        self.data.push(child)
    }
}
