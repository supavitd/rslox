use crate::token::Token;
use crate::lox;
use std::boxed::Box;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary(Box::new(left), operator, Box::new(right))
    }

    pub fn unary(operator: Token, expr: Expr) -> Self {
        Self::Unary(operator, Box::new(expr))
    }

    pub fn literal_num(num: f64) -> Self {
        Self::Literal(Value::Number(num))
    }

    pub fn literal_str(string: String) -> Self {
        Self::Literal(Value::String(string))
    }

    pub fn grouping(expr: Expr) -> Self {
        Self::Grouping(Box::new(expr))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

impl TryFrom<Value> for bool {
    type Error = lox::RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let coerced = match value {
            Value::True => true,
            Value::False => false,
            Value::Nil => false,
            Value::String(s) => s.is_empty(),
            Value::Number(n) => n == 0.0,
        };

        Ok(coerced)
    }
}

impl TryFrom<Value> for f64 {
    type Error = lox::RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Number(n) = value {
            Ok(n)
        } else {
            Err(Self::Error { message: "Not a number".to_string() })
        }
    }
}

impl TryFrom<Value> for String {
    type Error = lox::RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::String(s) = value {
            Ok(s)
        } else {
            Err(Self::Error { message: "Not a string".to_string()})
        }
    }
}
