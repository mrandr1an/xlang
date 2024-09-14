use std::{collections::HashMap, fmt::Display};

use crate::types::designators::TypeInstance;

use super::ast::{
    ASTKind, BinExpr, Expr, Func, FuncDef, Param, Return, UnaryExpr, Value, Variable, AST,
};

pub enum ScopeKind {
    Var,
    Param,
    Arg,
    FSign,
}

impl Display for ScopeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "Variable"),
            Self::Param => write!(f, "Parameter"),
            Self::Arg => write!(f, "Arguement"),
            Self::FSign => write!(f, "Signature"),
        }
    }
}

pub enum SymbolError {
    AlreadyExists(Symbol),
}

pub struct Symbol {
    _type: TypeInstance,
    kind: ScopeKind,
    val: Option<Value>,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_none() {
            write!(f, "{} | {} | None", self._type, self.kind)
        } else {
            write!(f, "{} | {} | {}", self._type, self.kind, self.val.unwrap())
        }
    }
}

impl Symbol {
    fn new(_type: TypeInstance, kind: ScopeKind, val: Option<Value>) -> Self {
        Self { _type, kind, val }
    }
}

pub type TableId = usize;

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    parent: Option<TableId>,
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name | Type | Scope | Value")?;
        for (key, value) in self.table.iter() {
            writeln!(f, "{} | {} ", key, value)?;
        }
        write!(f, "")
    }
}

impl SymbolTable {
    fn new(parent: Option<TableId>) -> Self {
        SymbolTable {
            table: HashMap::new(),
            parent,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.table.get(name)
    }

    fn insert_abstract(&mut self, kind: &ASTKind) -> Result<(), SymbolError> {
        match kind {
            ASTKind::Expr(expr) => self.insert_expr(expr),
            ASTKind::Param(param) => self.insert_param(param),
            ASTKind::Return(ret) => self.insert_return(ret),
            ASTKind::Val(val) => self.insert_val(val),
            ASTKind::VarDec(var) => self.insert_var(var),
            ASTKind::FuncDef(fdef) => self.insert_fdef(fdef),
            //Adds function def in curr and creates a new child with parent current
            ASTKind::Func(func) => self.insert_func(func),
        }
    }

    fn insert_val(&mut self, val: &Value) -> Result<(), SymbolError> {
        self.table.insert(
            self.gen_tmpname(),
            Symbol::new(val.get_type(), ScopeKind::Var, Some(*val)),
        );
        Ok(())
    }

    fn insert_param(&mut self, param: &Param) -> Result<(), SymbolError> {
        match self.table.insert(
            param.1.to_string(),
            Symbol::new(param.0.clone(), ScopeKind::Param, None),
        ) {
            None => Ok(()),
            Some(e) => Err(SymbolError::AlreadyExists(e)),
        }
    }

    fn insert_return(&mut self, ret: &Return) -> Result<(), SymbolError> {
        self.insert_abstract(&ret.0.kind)
    }

    fn insert_fdef(&mut self, fdef: &FuncDef) -> Result<(), SymbolError> {
        for param in fdef.2.iter() {
            match self.insert_param(&param) {
                Ok(()) => (),
                Err(e) => return Err(e),
            };
        }

        match self.table.insert(
            fdef.1.to_string(),
            Symbol::new(fdef.0.clone(), ScopeKind::FSign, None),
        ) {
            None => Ok(()),
            Some(e) => Err(SymbolError::AlreadyExists(e)),
        }
    }

    fn insert_func(&mut self, f: &Func) -> Result<(), SymbolError> {
        for ast in f.1.iter() {
            match self.insert_abstract(&ast.kind) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn insert_var(&mut self, var: &Variable) -> Result<(), SymbolError> {
        let _type = var.0.clone();
        let name = var.1;
        let expr = &var.2;
        match self.insert_expr(expr) {
            Ok(()) => {
                match self
                    .table
                    .insert(name.to_string(), Symbol::new(_type, ScopeKind::Var, None))
                {
                    None => Ok(()),
                    Some(e) => Err(SymbolError::AlreadyExists(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn insert_expr(&mut self, expr: &Expr) -> Result<(), SymbolError> {
        match expr {
            Expr::Binary(bin) => self.insert_bin_expr(bin),
            Expr::Noop(val) => self.insert_val(val),
            Expr::Unary(unary) => self.insert_unary(unary),
        }
    }

    fn insert_bin_expr(&mut self, bin: &BinExpr) -> Result<(), SymbolError> {
        let lhs = self.insert_abstract(&bin.lhs.kind);
        let rhs = self.insert_abstract(&bin.rhs.kind);
        match (lhs, rhs) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(e), Ok(())) => Err(e),
            (Ok(()), Err(e)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        }
    }

    fn insert_unary(&mut self, un: &UnaryExpr) -> Result<(), SymbolError> {
        match un {
            UnaryExpr::Id(_) => Ok(()),
        }
    }

    fn gen_tmpname(&self) -> String {
        let mut id = 0;
        let mut tmp_name = format!("t{}", id);
        while self.table.contains_key(&tmp_name) {
            id += 1;
            tmp_name = format!("t{}", id);
        }
        tmp_name
    }
}

pub struct SymbolMap {
    inner: Vec<SymbolTable>,
    pos: TableId,
}
impl Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, table) in self.inner.iter().enumerate() {
            writeln!(f, "POS: {}", id)?;
            writeln!(f, "{}", table)?;
        }
        Ok(())
    }
}
impl SymbolMap {
    fn new() -> Self {
        Self {
            inner: Vec::new(),
            pos: 0,
        }
    }

    fn add(&mut self, parent: Option<TableId>) -> TableId {
        self.inner.push(SymbolTable::new(parent));
        self.inner.len() - 1
    }

    fn insert(
        &mut self,
        curr: &mut SymbolTable,
        prev: Option<&mut SymbolTable>,
        kind: &ASTKind,
    ) -> Result<(), SymbolError> {
        match kind {
            ASTKind::Expr(expr) => curr.insert_expr(expr),
            ASTKind::Param(param) => curr.insert_param(param),
            ASTKind::Return(ret) => curr.insert_return(ret),
            ASTKind::Val(val) => curr.insert_val(val),
            ASTKind::VarDec(var) => curr.insert_var(var),
            ASTKind::FuncDef(fdef) => curr.insert_fdef(fdef),
            //Adds function def in curr and creates a new child with parent current
            ASTKind::Func(f) => {
                let sign = curr.insert_fdef(&f.0);
                let new_scope_ptr = self.add(Some(self.pos));
                let new_scope = self.inner.get_mut(new_scope_ptr).unwrap();
                new_scope.insert_func(f)
            }
        }
    }

    fn fill_from_source(&mut self, source: &[AST]) -> Result<(), SymbolError> {
        let mut root = SymbolTable::new(None);
        for ast in source.iter() {
            match self.insert(&mut root, None, &ast.kind) {
                Ok(()) => (),
                Err(e) => return Err(e),
            };
        }
        self.inner.insert(0, root);
        Ok(())
    }

    pub fn get(&self, id: TableId, name: &str) -> Option<&Symbol> {
        self.inner.get(id).unwrap().get(name)
    }

    pub fn get_from_val(&self, id: TableId, val: Value) -> Option<&String> {
        for (key, symbol) in self.inner.get(id).unwrap().table.iter() {
            if let Some(value) = symbol.val {
                if val == value {
                    return Some(key);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::symboltable::SymbolMap;
    use crate::frontend::{ast::Source, symboltable::SymbolTable};

    use crate::{
        frontend::ast::{
            BinExpr, BinOp, Expr, Func, FuncDef, Return, SignKind, Unsigned, Value, Variable, AST,
        },
        types::designators::TypeInstance,
    };

    use super::ASTKind;
    #[test]
    fn simple_table() {
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

        let main_ast = ASTKind::Func(Func(
            FuncDef(TypeInstance::Void, "main", Vec::new()),
            vec![var, ret_ast],
        ));

        let source = Source(vec![AST::new(main_ast)]);
        let mut s_map = SymbolMap::new();
        let _ = s_map.fill_from_source(&source.0);

        for ast in source.0 {
            println!("{}", ast.kind);
        }

        println!("{}", s_map);
    }
}
