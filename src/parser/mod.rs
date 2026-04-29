use crate::lexer::Token;
use crate::ast::{Expr, BinOp, Stmt, CompareOp};
use anyhow::{Result, bail};

pub struct Parser {
    tokens: Vec<(Token, String)>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, String)>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _)| t)
    }

    fn peek_str(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(|(_, s)| s.as_str())
    }

    fn consume(&mut self, expected: Token) -> Result<String> {
        if let Some((t, s)) = self.tokens.get(self.pos) {
            if *t == expected {
                self.pos += 1;
                return Ok(s.clone());
            }
        }
        bail!("Expected {:?}, found {:?}", expected, self.peek())
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self.pos < self.tokens.len() {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        match self.peek() {
            Some(Token::Ident) => {
                let ident = self.peek_str().unwrap().to_string();
                self.consume(Token::Ident)?;
                self.consume(Token::Assign)?;
                let expr = self.parse_expression()?;
                Ok(Stmt::Assign(ident, expr))
            }
            Some(Token::If) => {
                self.consume(Token::If)?;
                let cond = self.parse_expression()?;
                self.consume(Token::Then)?;
                let mut body = Vec::new();
                while let Some(t) = self.peek() {
                    if *t == Token::End { break; }
                    body.push(self.parse_statement()?);
                }
                self.consume(Token::End)?;
                Ok(Stmt::If(cond, body))
            }
            Some(Token::Loop) => {
                self.consume(Token::Loop)?;
                let count = self.parse_expression()?;
                self.consume(Token::Times)?;
                let mut body = Vec::new();
                while let Some(t) = self.peek() {
                    if *t == Token::End { break; }
                    body.push(self.parse_statement()?);
                }
                self.consume(Token::End)?;
                Ok(Stmt::Loop(count, body))
            }
            Some(Token::DrawPixel) => {
                self.consume(Token::DrawPixel)?;
                let x = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let y = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let color = self.parse_expression()?;
                Ok(Stmt::DrawPixel(x, y, color))
            }
            Some(Token::DrawRect) => {
                self.consume(Token::DrawRect)?;
                let x = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let y = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let w = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let h = self.parse_expression()?;
                self.consume(Token::Comma)?;
                let color = self.parse_expression()?;
                Ok(Stmt::DrawRect(x, y, w, h, color))
            }
            Some(Token::ClearScreen) => {
                self.consume(Token::ClearScreen)?;
                Ok(Stmt::ClearScreen)
            }
            Some(Token::Delay) => {
                self.consume(Token::Delay)?;
                let ms = self.parse_expression()?;
                Ok(Stmt::Delay(ms))
            }
            Some(Token::WaitKey) => {
                self.consume(Token::WaitKey)?;
                Ok(Stmt::WaitKey)
            }
            _ => bail!("Unexpected token: {:?}", self.peek()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_add_sub()?;
        while let Some(op) = self.peek() {
            let cmp_op = match op {
                Token::Greater => CompareOp::Greater,
                Token::Less    => CompareOp::Less,
                _ => break,
            };
            self.consume(*op)?;
            let right = self.parse_add_sub()?;
            left = Expr::Compare(Box::new(left), cmp_op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_add_sub(&mut self) -> Result<Expr> {
        let mut left = self.parse_mul_div()?;
        while let Some(op) = self.peek() {
            match op {
                Token::Plus => {
                    self.consume(Token::Plus)?;
                    let right = self.parse_mul_div()?;
                    left = Expr::Binary(Box::new(left), BinOp::Add, Box::new(right));
                }
                Token::Minus => {
                    self.consume(Token::Minus)?;
                    let right = self.parse_mul_div()?;
                    left = Expr::Binary(Box::new(left), BinOp::Sub, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.peek() {
            match op {
                Token::Star => {
                    self.consume(Token::Star)?;
                    let right = self.parse_primary()?;
                    left = Expr::Binary(Box::new(left), BinOp::Mul, Box::new(right));
                }
                Token::Slash => {
                    self.consume(Token::Slash)?;
                    let right = self.parse_primary()?;
                    left = Expr::Binary(Box::new(left), BinOp::Div, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.peek() {
            Some(Token::Minus) => {
                // Unary minus: -expr
                self.consume(Token::Minus)?;
                let expr = self.parse_primary()?;
                // Convert to 0 - expr
                let zero = Expr::Integer(0);
                Ok(Expr::Binary(Box::new(zero), BinOp::Sub, Box::new(expr)))
            }
            Some(Token::Integer) => {
                let s = self.consume(Token::Integer)?;
                let val = s.parse::<i32>()?;
                Ok(Expr::Integer(val))
            }
            Some(Token::Ident) => {
                let s = self.consume(Token::Ident)?;
                Ok(Expr::Variable(s))
            }
            Some(Token::LParen) => {
                self.consume(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }
            _ => bail!("Expected integer, identifier, or '('"),
        }
    }
}

pub fn parse_program(source: &str) -> Result<Vec<Stmt>> {
    let tokens = crate::lexer::tokenize(source);
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}