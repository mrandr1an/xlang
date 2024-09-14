use super::{
    ast::{ASTKind, BinExpr, BinOp, Expr, Source, UnaryExpr, Value},
    symboltable::{Symbol, SymbolError, SymbolMap},
};

pub enum Instruction<'a> {
    BAssign(BinAssign<'a>),
    SAssign(SingleAssign<'a>),
    Mov(Move<'a>),
    Goto(Goto<'a>),
    Call(Call<'a>),
}

impl<'a> Instruction<'a> {}

pub struct BinAssign<'a> {
    lhs: &'a str,
    lop: &'a str,
    op: BinOp,
    rop: &'a str,
}

pub struct SingleAssign<'a> {
    lhs: &'a str,
    rhs: &'a Symbol,
}

pub struct Move<'a> {
    lhs: &'a str,
    rhs: &'a str,
}

pub struct Goto<'a> {
    id: BlockId<'a>,
}

pub struct Call<'a> {
    id: BlockId<'a>,
}

pub struct DebugPrint {
    val: String,
}

pub type BlockId<'a> = &'a str;

pub struct BasicBlock<'a> {
    prev: Option<Vertices<'a>>,
    instrs: Vec<Instruction<'a>>,
    next: Option<Vertices<'a>>,
    id: BlockId<'a>,
}

impl<'a> BasicBlock<'a> {
    fn new(prev: Option<Vertices<'a>>, name: &'a str) -> Self {
        Self {
            prev,
            instrs: Vec::new(),
            next: None,
            id: name,
        }
    }

    fn entry() -> Self {
        Self {
            prev: None,
            instrs: Vec::new(),
            next: None,
            id: "entry",
        }
    }

    fn add(&mut self, instr: Instruction<'a>) {
        self.instrs.push(instr)
    }
}

pub enum Vertices<'a> {
    Linear(BlockId<'a>),
    Branch(Vec<BlockId<'a>>),
}

pub struct Cfg<'a> {
    blocks: Vec<BasicBlock<'a>>,
    map: SymbolMap,
    pos: usize,
}

impl<'a> Cfg<'a> {
    fn new(map: SymbolMap) -> Self {
        Self {
            blocks: Vec::new(),
            map,
            pos: 0,
        }
    }

    fn new_block(&mut self, prev: Option<Vertices>) -> BlockId {
        todo!()
    }

    fn fold_ast(&mut self, ast: ASTKind, block: &mut BasicBlock) {
        match ast {
            ASTKind::Val(val) => todo!(),
            ASTKind::VarDec(var) => todo!(),
            ASTKind::Expr(expr) => todo!(),
            ASTKind::Return(ret) => todo!(),
            ASTKind::FuncDef(fdef) => todo!(),
            ASTKind::Param(param) => todo!(),
            ASTKind::Func(func) => todo!(),
        }
    }

    fn fold_expr(&'a mut self, expr: Expr, block: &'a mut BasicBlock<'a>) {
        todo!()
    }

    fn fold_val(&'a mut self, val: Value, block: &'a mut BasicBlock<'a>) {
        let name = self.map.get_from_val(self.pos, val).unwrap();
        let val = self.map.get(self.pos, name).unwrap();
        let ins = SingleAssign {
            lhs: name,
            rhs: val,
        };
        block.add(Instruction::SAssign(ins));
    }

    fn fill_from_source(&mut self, source: Source) {
        let mut entry = BasicBlock::entry();
        for ast in source.0.into_iter() {
            match ast.kind {
                ASTKind::Expr(expr) => todo!(),
                ASTKind::Param(param) => todo!(),
                ASTKind::Return(ret) => todo!(),
                ASTKind::Val(val) => todo!(),
                ASTKind::VarDec(var) => todo!(),
                ASTKind::FuncDef(fdef) => todo!(),
                ASTKind::Func(func) => todo!(),
            }
        }
    }
}
