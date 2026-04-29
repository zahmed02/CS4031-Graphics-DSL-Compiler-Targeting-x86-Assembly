use std::collections::HashMap;
use crate::ast::{Expr, Stmt};
use anyhow::{Result, bail};

pub struct SymbolTable {
    scopes: Vec<HashMap<String, VarType>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: &str) {
        let current = self.scopes.last_mut().unwrap();
        if !current.contains_key(name) {
            current.insert(name.to_string(), VarType::Int);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<VarType> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}

pub fn semantic_check(program: &[Stmt]) -> Result<()> {
    let mut sym = SymbolTable::new();
    for stmt in program {
        check_stmt(stmt, &mut sym)?;
    }
    Ok(())
}

fn check_stmt(stmt: &Stmt, sym: &mut SymbolTable) -> Result<()> {
    match stmt {
        Stmt::Assign(var, expr) => {
            if sym.lookup(var).is_none() {
                sym.declare(var);
            }
            let expr_ty = check_expr(expr, sym)?;
            if expr_ty != VarType::Int {
                bail!("Assignment type mismatch");
            }
            Ok(())
        }
        Stmt::If(cond, body) => {
            let cond_ty = check_expr(cond, sym)?;
            if cond_ty != VarType::Int {
                bail!("Condition must be integer");
            }
            sym.enter_scope();
            for s in body {
                check_stmt(s, sym)?;
            }
            sym.exit_scope();
            Ok(())
        }
        Stmt::Loop(count, body) => {
            let count_ty = check_expr(count, sym)?;
            if count_ty != VarType::Int {
                bail!("Loop count must be integer");
            }
            sym.enter_scope();
            for s in body {
                check_stmt(s, sym)?;
            }
            sym.exit_scope();
            Ok(())
        }
        Stmt::DrawPixel(x, y, color) => {
            check_expr(x, sym)?;
            check_expr(y, sym)?;
            check_expr(color, sym)?;
            Ok(())
        }
        Stmt::DrawRect(x, y, w, h, color) => {
            check_expr(x, sym)?;
            check_expr(y, sym)?;
            check_expr(w, sym)?;
            check_expr(h, sym)?;
            check_expr(color, sym)?;
            Ok(())
        }
        Stmt::ClearScreen => Ok(()),
        Stmt::Delay(ms) => {
            check_expr(ms, sym)?; // ensure the expression type checks, then ignore result
            Ok(())
        }
        Stmt::WaitKey => Ok(()),
    }
}

fn check_expr(expr: &Expr, sym: &SymbolTable) -> Result<VarType> {
    match expr {
        Expr::Integer(_) => Ok(VarType::Int),
        Expr::Variable(name) => {
            if sym.lookup(name).is_some() {
                Ok(VarType::Int)
            } else {
                bail!("Undeclared variable '{}'", name)
            }
        }
        Expr::Binary(left, _, right) => {
            check_expr(left, sym)?;
            check_expr(right, sym)?;
            Ok(VarType::Int)
        }
        Expr::Compare(left, _, right) => {
            check_expr(left, sym)?;
            check_expr(right, sym)?;
            Ok(VarType::Int)
        }
    }
}