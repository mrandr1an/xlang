use std::fmt::Display;

use crate::types::designators::TypeInstance;

pub struct Source<'a>(pub Vec<AST<'a>>);

pub struct AST<'a> {
    pub kind: ASTKind<'a>,
}

impl<'a> AST<'a> {
    pub fn new(kind: ASTKind<'a>) -> Self {
        Self { kind }
    }
}

pub enum ASTKind<'a> {
    Val(Value),
    VarDec(Variable<'a>),
    Param(Param<'a>),
    Expr(Expr<'a>),
    FuncDef(FuncDef<'a>),
    Func(Func<'a>),
    Return(Return<'a>),
}

impl<'a> Display for ASTKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTKind::Expr(expr) => write!(f, "{}", expr),
            ASTKind::Val(val) => write!(f, "{}", val),
            ASTKind::VarDec(var) => write!(f, "{}", var),
            ASTKind::FuncDef(fdef) => write!(f, "{}", fdef),
            ASTKind::Return(ret) => write!(f, "{}", ret),
            ASTKind::Param(param) => write!(f, "{}", param),
            ASTKind::Func(func) => write!(f, "{}", func),
        }
    }
}

/// Values that have no associated identifiers with them by default
/// Like numbers and string literals.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Value {
    Integer(SignKind),
}

impl Value {
    pub fn get_type(&self) -> TypeInstance {
        match self {
            Value::Integer(SignKind::Signed(Signed::Char(_))) => TypeInstance::Char,
            Value::Integer(SignKind::Signed(Signed::Int(_))) => TypeInstance::Int,
            Value::Integer(SignKind::Signed(Signed::Short(_))) => TypeInstance::Short,
            Value::Integer(SignKind::Signed(Signed::Long(_))) => TypeInstance::Long,
            Value::Integer(SignKind::Signed(Signed::LongLong(_))) => TypeInstance::LongLong,
            Value::Integer(SignKind::Unsigned(Unsigned::Char(_))) => TypeInstance::Char,
            Value::Integer(SignKind::Unsigned(Unsigned::Int(_))) => TypeInstance::Int,
            Value::Integer(SignKind::Unsigned(Unsigned::Short(_))) => TypeInstance::Short,
            Value::Integer(SignKind::Unsigned(Unsigned::Long(_))) => TypeInstance::Long,
            Value::Integer(SignKind::Unsigned(Unsigned::LongLong(_))) => TypeInstance::LongLong,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Signed {
    Char(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    LongLong(i64),
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Unsigned {
    Char(u8),
    Short(u16),
    Int(u32),
    Long(u64),
    LongLong(u64),
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum SignKind {
    Signed(Signed),
    Unsigned(Unsigned),
}

impl Display for Signed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signed::Char(c) => write!(f, "{c}"),
            Signed::Short(c) => write!(f, "{c}"),
            Signed::Int(c) => write!(f, "{c}"),
            Signed::Long(c) => write!(f, "{c}"),
            Signed::LongLong(c) => write!(f, "{c}"),
        }
    }
}

impl Display for Unsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unsigned::Char(c) => write!(f, "{c}"),
            Unsigned::Short(c) => write!(f, "{c}"),
            Unsigned::Int(c) => write!(f, "{c}"),
            Unsigned::Long(c) => write!(f, "{c}"),
            Unsigned::LongLong(c) => write!(f, "{c}"),
        }
    }
}

impl Display for SignKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignKind::Signed(i) => write!(f, "{i}"),
            SignKind::Unsigned(i) => write!(f, "{i}"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            _ => todo!(),
        }
    }
}

/// Values or Expressions of values that are assosiated with an identifier
/// in the source language
/// ex. int a = 42 + 3 - b;
pub struct Variable<'a>(pub TypeInstance, pub &'a str, pub Expr<'a>);
impl<'a> Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} = {};", self.0, self.1, self.2)
    }
}

///Expressions are collections of values and/or variables that are not
///necessarily assosiated with an identifier but might be.
pub enum Expr<'a> {
    Binary(BinExpr<'a>),
    Unary(UnaryExpr<'a>),
    Noop(Value),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(bin) => write!(f, "{bin}"),
            Expr::Unary(un) => write!(f, "{un}"),
            Expr::Noop(no) => write!(f, "{no}"),
        }
    }
}

pub struct BinExpr<'a> {
    pub lhs: Box<AST<'a>>,
    pub op: BinOp,
    pub rhs: Box<AST<'a>>,
}

impl<'a> Display for BinExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs.kind, self.op, self.rhs.kind)
    }
}

pub enum UnaryExpr<'a> {
    Id(Id<'a>),
}

impl<'a> Display for UnaryExpr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryExpr::Id(id) => write!(f, "{}", id),
            _ => todo!(),
        }
    }
}

pub struct Id<'a>(&'a str);

impl<'a> Display for Id<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl<'a> Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
        }
    }
}

pub struct Param<'a>(pub TypeInstance, pub &'a str);

impl<'a> Display for Param<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
pub struct FuncDef<'a>(pub TypeInstance, pub &'a str, pub Vec<Param<'a>>);

impl<'a> Display for FuncDef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}(", self.0, self.1);
        for param in self.2.iter() {
            write!(f, "{},", param);
        }
        writeln!(f, ");")
    }
}

pub struct Func<'a>(pub FuncDef<'a>, pub Vec<AST<'a>>);

impl<'a> Display for Func<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {{", self.0);
        for ast in self.1.iter() {
            writeln!(f, "{}", ast.kind)?;
        }
        writeln!(f, "}}")
    }
}
pub struct Return<'a>(pub Box<AST<'a>>);

impl<'a> Display for Return<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {};", self.0.kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        frontend::ast::{
            BinExpr, BinOp, Expr, Func, FuncDef, Return, SignKind, Unsigned, Value, Variable, AST,
        },
        types::designators::TypeInstance,
    };

    use super::ASTKind;

    #[test]
    fn print_simple_cprogram() {
        let expr = Expr::Binary(BinExpr {
            lhs: Box::new(AST::new(ASTKind::Val(Value::Integer(SignKind::Unsigned(
                Unsigned::Int(5),
            ))))),
            op: BinOp::Add,
            rhs: Box::new(AST::new(ASTKind::Val(Value::Integer(SignKind::Unsigned(
                Unsigned::Int(10),
            ))))),
        });

        let var = AST::new(ASTKind::VarDec(Variable(TypeInstance::Int, "a", expr)));
        let ret = Return(Box::new(AST {
            kind: ASTKind::Val(Value::Integer(SignKind::Unsigned(Unsigned::Int(0)))),
        }));

        let ret_ast = AST {
            kind: ASTKind::Return(ret),
        };

        let ast = ASTKind::Func(Func(
            FuncDef(TypeInstance::Void, "main", Vec::new()),
            vec![var, ret_ast],
        ));
        println!("{}", ast);
    }
}
