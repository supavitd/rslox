use crate::token::Token;
use std::boxed::Box;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(Box::new(left), operator, Box::new(right))
    }

    pub fn unary(operator: Token, expr: Expr) -> Self {
        Self::Unary(operator, Box::new(expr))
    }

    pub fn literal_num(num: f64) -> Self {
        Self::Literal(Literal::Number(num))
    }

    pub fn literal_str(string: String) -> Self {
        Self::Literal(Literal::String(string))
    }

    pub fn grouping(expr: Expr) -> Self {
        Self::Grouping(Box::new(expr))
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}
