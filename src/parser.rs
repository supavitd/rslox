use crate::expr::{Expr, Value};
use crate::token::Token;
use crate::token_type::TokenType;
use std::fmt;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

pub type ParseResult = Result<Expr, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error...")
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expression()
    }

    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult {
        let mut expr = self.comparison()?;

        while let Some(next) = self.peek() {
            if !matches!(next.type_, TokenType::BangEqual | TokenType::EqualEqual) {
                break;
            }
            let token = self.advance().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, token, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult {
        let mut expr = self.term()?;
        while let Some(next) = self.peek() {
            if !matches!(
                next.type_,
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            ) {
                break;
            }
            let token = self.advance().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, token, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParseResult {
        let mut expr = self.factor()?;
        while let Some(next) = self.peek() {
            if !matches!(next.type_, TokenType::Plus | TokenType::Minus) {
                break;
            }
            let token = self.advance().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, token, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult {
        let mut expr = self.unary()?;
        while let Some(next) = self.peek() {
            if !matches!(next.type_, TokenType::Slash | TokenType::Star) {
                break;
            }
            let token = self.advance().clone();
            let right = self.unary()?;
            expr = Expr::binary(expr, token, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult {
        let Some(next) = self.peek() else {
            unimplemented!();
        };

        if matches!(next.type_, TokenType::Bang | TokenType::Minus) {
            let token = self.advance().clone();
            let expr = self.unary()?;
            return Ok(Expr::unary(token, expr));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult {
        let token = self.advance();
        let expr = match &token.type_ {
            TokenType::Number(num) => Expr::literal_num(*num),
            TokenType::String(string) => Expr::literal_str(string.clone()),
            TokenType::True => Expr::Literal(Value::True),
            TokenType::False => Expr::Literal(Value::False),
            TokenType::Nil => Expr::Literal(Value::Nil),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                if !matches!(self.advance().type_, TokenType::RightParen) {
                    return Err(ParseError { message: "Expected ')' after expression".to_string()});
                }
                Expr::grouping(expr)
            }
            _ => { return Err(ParseError { message: "Expected expression.".to_string() }); }
        };

        Ok(expr)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() {
            return matches!(token.type_, TokenType::Eof);
        }
        true
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        if !self.is_at_end() {
            self.current += 1;
        }
        token
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {

        }
    }
}
