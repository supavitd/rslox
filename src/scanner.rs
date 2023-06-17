use std::collections::HashMap;
use std::sync::OnceLock;

use crate::error::Error;
use crate::token::Token;
use crate::token_type::TokenType;

pub static KEYWORDS: OnceLock<HashMap<&str, TokenType>> = OnceLock::new();

pub fn init() {
    KEYWORDS.get_or_init(|| {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    });
}

#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: u32,
    source_chars: Vec<char>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let source_chars = String::from(&source).chars().collect();
        Self {
            source,
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
            source_chars: source_chars,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let next = self.source_chars[self.current];
        self.current += 1;
        next
    }

    fn peek(&self) -> char {
        self.peek_next(0)
    }

    fn peek_next(&self, lookahead: usize) -> char {
        if self.current + lookahead >= self.source.len() {
            '\0'
        } else {
            self.source_chars[self.current + lookahead]
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        let is_a_match = !self.is_at_end() && self.source_chars[self.current] == expected;

        if is_a_match {
            self.current += 1;
        }

        is_a_match
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let type_ = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(type_)
            }
            '>' => {
                let type_ = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(type_)
            }
            '<' => {
                let type_ = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(type_)
            }
            '/' if self.match_next('/') => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '/' => self.add_token(TokenType::Slash),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.scan_string()?,
            c if c.is_digit(10) => self.scan_number()?,
            c if c.is_alphanumeric() => self.scan_kw_or_identifier()?,
            _ => return Err(Error::new("Unexpected character.", self.line)),
        }

        Ok(())
    }

    fn scan_string(&mut self) -> Result<(), Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::new("Unterminated string.", self.line));
        }

        self.advance();

        let value = String::from_iter(self.source_chars[self.start + 1..self.current - 1].iter());
        self.add_token(TokenType::String(value));

        Ok(())
    }

    fn scan_number(&mut self) -> Result<(), Error> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next(1).is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = String::from_iter(self.source_chars[self.start..self.current].iter());
        let Ok(number) = value.parse::<f64>() else {
            return Err(Error::new("Invalid number", self.line));
        };

        self.add_token(TokenType::Number(number));

        Ok(())
    }

    fn scan_kw_or_identifier(&mut self) -> Result<(), Error> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = String::from_iter(self.source_chars[self.start..self.current].iter());

        match KEYWORDS
            .get()
            .expect("scanner::init must be called before using Scanner")
            .get(&String::as_str(&text))
        {
            Some(token_type) => self.add_token(token_type.clone()),
            None => self.add_token(TokenType::Identifier),
        }

        Ok(())
    }

    fn add_token(&mut self, type_: TokenType) {
        let token = Token {
            type_,
            lexeme: String::from_iter(self.source_chars[self.start..self.current].iter()),
            line: self.line,
        };
        self.tokens.push(token);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::matches;

    #[test]
    #[should_panic]
    #[ignore = "Not isolated with other tests so would fail sometimes"]
    fn test_use_before_init() {
        let _ = Scanner::new(String::from("while")).scan_kw_or_identifier();
    }

    #[test]
    fn test_init_keywords_lookup() {
        init();
        assert!(KEYWORDS.get().is_some());
    }

    #[test]
    fn test_keyword() -> Result<(), Error> {
        init();
        let mut scanner = Scanner::new(String::from("while"));
        scanner.scan_kw_or_identifier()?;
        assert_eq!(scanner.tokens.len(), 1);
        assert!(matches!(scanner.tokens[0].type_, TokenType::While));

        Ok(())
    }
}
