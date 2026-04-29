use crate::ast::{Expr, Stmt, BinOp, CompareOp};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum IRInst {
    // Assignment: dest = src
    Copy(String, String),
    // Binary: dest = src1 op src2
    BinOp(String, BinOp, String, String),
    // Comparison: dest = (left cmp right)  (result 0 or 1)
    Compare(String, CompareOp, String, String),
    // Load constant: dest = val
    LoadConst(String, i32),
    // Drawing functions
    DrawPixel(String, String, String),
    DrawRect(String, String, String, String, String),
    ClearScreen,
    Delay(String),
    WaitKey,
    // Control flow
    Label(String),
    Jump(String),
    JumpIfZero(String, String),
}

pub struct IRGenerator {
    temp_counter: usize,
    label_counter: usize,
    pub code: Vec<IRInst>,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            code: Vec::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let t = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        t
    }

    fn new_label(&mut self) -> String {
        let l = format!("L{}", self.label_counter);
        self.label_counter += 1;
        l
    }

    pub fn generate(&mut self, stmts: &[Stmt]) -> Result<()> {
        for stmt in stmts {
            self.gen_stmt(stmt)?;
        }
        Ok(())
    }

    fn gen_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Assign(var, expr) => {
                let val = self.gen_expr(expr)?;
                self.code.push(IRInst::Copy(var.clone(), val));
                Ok(())
            }
            Stmt::If(cond, body) => {
                let cond_val = self.gen_expr(cond)?;
                let else_label = self.new_label();
                let end_label = self.new_label();
                self.code.push(IRInst::JumpIfZero(cond_val, else_label.clone()));
                for s in body {
                    self.gen_stmt(s)?;
                }
                self.code.push(IRInst::Jump(end_label.clone()));
                self.code.push(IRInst::Label(else_label));
                self.code.push(IRInst::Label(end_label));
                Ok(())
            }
            Stmt::Loop(count, body) => {
                let count_val = self.gen_expr(count)?;
                let loop_label = self.new_label();
                let counter = self.new_temp();
                self.code.push(IRInst::LoadConst(counter.clone(), 0));
                self.code.push(IRInst::Label(loop_label.clone()));
                let cond_temp = self.new_temp();
                self.code.push(IRInst::BinOp(cond_temp.clone(), BinOp::Sub, counter.clone(), count_val.clone()));
                let exit_label = self.new_label();
                self.code.push(IRInst::JumpIfZero(cond_temp, exit_label.clone()));
                for s in body {
                    self.gen_stmt(s)?;
                }
                let inc_temp = self.new_temp();
                self.code.push(IRInst::LoadConst(inc_temp.clone(), 1));
                let new_counter = self.new_temp();
                self.code.push(IRInst::BinOp(new_counter.clone(), BinOp::Add, counter.clone(), inc_temp));
                self.code.push(IRInst::Copy(counter, new_counter));
                self.code.push(IRInst::Jump(loop_label));
                self.code.push(IRInst::Label(exit_label));
                Ok(())
            }
            Stmt::DrawPixel(x, y, color) => {
                let xv = self.gen_expr(x)?;
                let yv = self.gen_expr(y)?;
                let cv = self.gen_expr(color)?;
                self.code.push(IRInst::DrawPixel(xv, yv, cv));
                Ok(())
            }
            Stmt::DrawRect(x, y, w, h, color) => {
                let xv = self.gen_expr(x)?;
                let yv = self.gen_expr(y)?;
                let wv = self.gen_expr(w)?;
                let hv = self.gen_expr(h)?;
                let cv = self.gen_expr(color)?;
                self.code.push(IRInst::DrawRect(xv, yv, wv, hv, cv));
                Ok(())
            }
            Stmt::ClearScreen => {
                self.code.push(IRInst::ClearScreen);
                Ok(())
            }
            Stmt::Delay(ms) => {
                let mv = self.gen_expr(ms)?;
                self.code.push(IRInst::Delay(mv));
                Ok(())
            }
            Stmt::WaitKey => {
                self.code.push(IRInst::WaitKey);
                Ok(())
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> Result<String> {
        match expr {
            Expr::Integer(i) => {
                let t = self.new_temp();
                self.code.push(IRInst::LoadConst(t.clone(), *i));
                Ok(t)
            }
            Expr::Variable(v) => Ok(v.clone()),
            Expr::Binary(l, op, r) => {
                let lv = self.gen_expr(l)?;
                let rv = self.gen_expr(r)?;
                let t = self.new_temp();
                self.code.push(IRInst::BinOp(t.clone(), op.clone(), lv, rv));
                Ok(t)
            }
            Expr::Compare(l, op, r) => {
                let lv = self.gen_expr(l)?;
                let rv = self.gen_expr(r)?;
                let t = self.new_temp();
                self.code.push(IRInst::Compare(t.clone(), op.clone(), lv, rv));
                Ok(t)
            }
        }
    }
}