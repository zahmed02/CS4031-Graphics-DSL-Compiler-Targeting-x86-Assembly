use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Integer(i32),
    Variable(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Compare(Box<Expr>, CompareOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOp {
    Less,
    Greater,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign(String, Expr),
    If(Expr, Vec<Stmt>),
    Loop(Expr, Vec<Stmt>),
    DrawPixel(Expr, Expr, Expr),
    DrawRect(Expr, Expr, Expr, Expr, Expr),
    ClearScreen,
    Delay(Expr),
    WaitKey,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Integer(i) => write!(f, "{}", i),
            Expr::Variable(v) => write!(f, "{}", v),
            Expr::Binary(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Compare(l, op, r) => write!(f, "({} {:?} {})", l, op, r),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
        };
        write!(f, "{}", s)
    }
}