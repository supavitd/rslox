use crate::expr::{Expr, Literal};
use crate::token::Token;
use crate::token_type::TokenType;
use std::fmt;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error...")
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        Some(self.expression())
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while matches!(
            self.peek().type_,
            TokenType::BangEqual | TokenType::EqualEqual
        ) {
            let token = self.advance().clone();
            let right = self.comparison();
            expr = Expr::binary(expr, token, right);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while matches!(
            self.peek().type_,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let token = self.advance().clone();
            let right = self.term();
            expr = Expr::binary(expr, token, right);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while matches!(self.peek().type_, TokenType::Plus | TokenType::Minus) {
            let token = self.advance().clone();
            let right = self.factor();
            expr = Expr::binary(expr, token, right);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while matches!(self.peek().type_, TokenType::Slash | TokenType::Star) {
            let token = self.advance().clone();
            let right = self.unary();
            expr = Expr::binary(expr, token, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if matches!(self.peek().type_, TokenType::Bang | TokenType::Minus) {
            let token = self.advance().clone();
            Expr::unary(token, self.unary())
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.advance();
        match &token.type_ {
            TokenType::Number(num) => Expr::literal_num(*num),
            TokenType::String(string) => Expr::literal_str(string.clone()),
            TokenType::True => Expr::Literal(Literal::True),
            TokenType::False => Expr::Literal(Literal::False),
            TokenType::Nil => Expr::Literal(Literal::Nil),
            TokenType::LeftParen => {
                let expr = self.expression();
                if !matches!(self.advance().type_, TokenType::RightParen) {
                    panic!("To be fixed later.");
                }
                Expr::grouping(expr)
            }
            _ => panic!("To be fixed later"),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().type_, TokenType::Eof)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}
