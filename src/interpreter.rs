use crate::expr::{Expr, Value};
use crate::lox;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn evaluate(expr: &Expr) -> Result<Value, lox::RuntimeError> {
        match expr {
            Expr::Literal(Value) => Ok(Value.clone()),
            Expr::Grouping(expr) => Self::evaluate(expr),
            Expr::Unary(token, expr) => {
                let val = Self::evaluate(expr)?;
                match token.type_ {
                    TokenType::Bang => {
                        if !bool::try_from(val)? {
                            Ok(Value::True)
                        } else {
                            Ok(Value::False)
                        }
                    }
                    TokenType::Minus => Ok(Value::Number(-f64::try_from(val)?)),
                    _ => Err(lox::RuntimeError {
                        message: String::from("Invalid token?"),
                    }),
                }
            }
            Expr::Binary(left, token, right) => Self::evaluate_binary(left, token, right),
        }
    }

    fn evaluate_binary(
        left: &Expr,
        token: &Token,
        right: &Expr,
    ) -> Result<Value, lox::RuntimeError> {
        let left_val = Self::evaluate(left)?;
        let right_val = Self::evaluate(right)?;

        match token.type_ {
            TokenType::Greater => {
                if f64::try_from(left_val)? > f64::try_from(right_val)? {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::GreaterEqual => {
                if f64::try_from(left_val)? >= f64::try_from(right_val)? {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::Less => {
                if f64::try_from(left_val)? < f64::try_from(right_val)? {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::LessEqual => {
                if f64::try_from(left_val)? <= f64::try_from(right_val)? {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::EqualEqual => {
                if left_val == right_val {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::BangEqual => {
                if left_val != right_val {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            TokenType::Minus => Ok(Value::Number(
                f64::try_from(left_val)? - f64::try_from(right_val)?,
            )),
            TokenType::Slash => Ok(Value::Number(
                f64::try_from(left_val)? / f64::try_from(right_val)?,
            )),
            TokenType::Star => Ok(Value::Number(
                f64::try_from(left_val)? * f64::try_from(right_val)?,
            )),
            TokenType::Plus => {
                match (left_val, right_val) {
                    (Value::Number(left_num), Value::Number(right_num)) => Ok(Value::Number(left_num + right_num)),
                    (Value::String(left_str), Value::String(right_str)) => Ok(Value::String(left_str + &right_str)),
                    _ => Err(lox::RuntimeError { message: String::from("Invalid operation") }),
                }
            }
            _ => Err(lox::RuntimeError {
                message: String::from("Invalid operation"),
            }),
        }
    }
}
