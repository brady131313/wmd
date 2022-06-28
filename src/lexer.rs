use crate::ast::{Quantity, TimeUnit, Unit};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single character tokens
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,
    Quantity,

    // Keywords
    And,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    True,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {}

#[derive(Debug, Clone)]
pub struct Token<'source> {
    typ: TokenType,
    lexeme: &'source str,
    literal: Option<TokenLiteral>,
    line: usize,
}

pub struct Lexer<'source> {
    src: &'source str,
    tokens: Vec<Token<'source>>,
    start: usize,   // First character in lexeme being scanned
    current: usize, // character currently being considered
    line: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(src: &'source str) -> Self {
        Self {
            src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'source>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token {
            typ: TokenType::Eof,
            lexeme: "",
            literal: None,
            line: self.line,
        });
        self.tokens
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        let end = self.current.min(self.src.len());
        dbg!(&self.src[self.current..end])
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            "(" => self.add_token(TokenType::LParen),
            _ => unimplemented!(),
        }
    }

    fn add_token(&mut self, typ: TokenType) {
        self.add_token_with_literal(typ, None)
    }

    fn add_token_with_literal(&mut self, typ: TokenType, literal: Option<TokenLiteral>) {
        let lexeme = &self.src[self.start..self.current];
        self.tokens.push(Token {
            typ,
            lexeme,
            literal,
            line: self.line,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        // let wmd_content = r#"3x rpe(8) 30.5s "this a string" fn if else true
        //     nil 5.0 10 false + - / * != ! == > >= < <= = and or
        //
        //     "#;
        let wmd_content = "(";
        let lexer = Lexer::new(&wmd_content);
        let tokens = lexer.scan_tokens();
        println!("{tokens:#?}");
        panic!()
    }
}
