use crate::{ast::Quantity, reporting::ErrorReporter};

macro_rules! is_digit {
    () => {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    };
}

#[rustfmt::skip]
macro_rules! is_alpha {
    () => {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" |
        "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" |
        "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "_"
    }
}

macro_rules! is_alpha_numeric {
    () => {
        is_alpha!() | is_digit!()
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    SemiColon,
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
pub enum TokenLiteral {
    String(String),
    Number(f64),
    Quantity(Quantity),
}

#[derive(Debug, Clone)]
pub struct Token<'source> {
    pub typ: TokenType,
    pub lexeme: &'source str,
    pub literal: Option<TokenLiteral>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpToken {
    pub typ: TokenType,
    pub line: usize,
}

impl<'source> From<&Token<'source>> for OpToken {
    fn from(token: &Token<'source>) -> Self {
        OpToken {
            typ: token.typ,
            line: token.line,
        }
    }
}

pub struct Lexer<'source, R> {
    src: &'source str,
    tokens: Vec<Token<'source>>,
    start: usize,   // First character in lexeme being scanned
    current: usize, // character currently being considered
    line: usize,
    reporter: R,
}

impl<'source, R: ErrorReporter> Lexer<'source, R> {
    pub fn new(src: &'source str, reporter: R) -> Self {
        Self {
            src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            reporter,
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
        let new_current = self.current + 1;
        let slice = &self.src[self.current..new_current];
        self.current = new_current;
        slice
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            "(" => self.add_token(TokenType::LParen),
            ")" => self.add_token(TokenType::RParen),
            "{" => self.add_token(TokenType::LBrace),
            "}" => self.add_token(TokenType::RBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::SemiColon),
            "*" => self.add_token(TokenType::Star),
            "!" => {
                if self.matches("=") {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            "=" => {
                if self.matches("=") {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            "<" => {
                if self.matches("=") {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            ">" => {
                if self.matches("=") {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            "/" => {
                if self.matches("/") {
                    while self.peek() != Some("\n") && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            " " | "\r" | "\t" => {}
            "\n" => self.line += 1,
            "\"" => self.string(),
            is_digit!() => self.number(),
            is_alpha!() => self.identifier(),
            _ => self.reporter.error(self.line, "Unexpected character."),
        }
    }

    fn identifier(&mut self) {
        while let Some(is_alpha_numeric!()) = self.peek() {
            self.advance();
        }

        // Check if matches keyword
        let text = &self.src[self.start..self.current];
        let typ = match text {
            "and" => TokenType::And,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fn" => TokenType::Fn,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "true" => TokenType::True,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(typ)
    }

    fn number(&mut self) {
        while let Some(is_digit!()) = self.peek() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == Some(".") && matches!(self.peek_next(), Some(is_digit!())) {
            // Consume the "."
            self.advance();

            while let Some(is_digit!()) = self.peek() {
                self.advance();
            }
        }

        // Look for unit
        if let Some(u @ ("s" | "m" | "%" | "x")) = self.peek() {
            let unit = u.parse().unwrap();
            // Consume the unit
            self.advance();

            let number = &self.src[self.start..self.current - 1];
            self.add_token_with_literal(
                TokenType::Quantity,
                Some(TokenLiteral::Quantity(Quantity::new(
                    number.parse().unwrap(),
                    unit,
                ))),
            )
        } else {
            let number = &self.src[self.start..self.current];
            self.add_token_with_literal(
                TokenType::Number,
                Some(TokenLiteral::Number(number.parse().unwrap())),
            )
        }
    }

    /// TODO: If supporting escape sequences like \n, unescape here
    fn string(&mut self) {
        while self.peek() != Some("\"") && !self.is_at_end() {
            if self.peek() == Some("\n") {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            unimplemented!("unterminated string")
        }

        // The closing "
        self.advance();

        // Trim surrounding quotes
        let string = &self.src[self.start + 1..self.current - 1];
        self.add_token_with_literal(TokenType::String, Some(TokenLiteral::String(string.into())))
    }

    fn matches(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        let slice = &self.src[self.current..self.current + 1];
        if slice != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<&str> {
        if self.is_at_end() {
            None
        } else {
            let slice = &self.src[self.current..self.current + 1];
            Some(slice)
        }
    }

    fn peek_next(&self) -> Option<&str> {
        if self.current + 1 >= self.src.len() {
            None
        } else {
            let slice = &self.src[self.current + 1..self.current + 2];
            Some(slice)
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
    use crate::reporting::StdoutReporter;

    use super::*;

    #[test]
    fn lexer() {
        // let wmd_content = r#"3x rpe(8) 30.5s "this a string" fn if else true
        //     nil 5.0 10 false + - / * != ! == > >= < <= = and or
        //
        //     "#;
        let wmd_content = r#"// this is a comment
            (( )){} // grouping stuff
            !*+-/=<> <= == // operators
            "this is a string"
            123 + 5.55
            ident_test
            and else false
            for fn if nil or
            true while
            30s 2m 30x 50%
            "#;
        let reporter = StdoutReporter;
        let lexer = Lexer::new(&wmd_content, &reporter);
        let tokens = lexer.scan_tokens();
        println!("{tokens:#?}");
        panic!()
    }
}
