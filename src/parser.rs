use crate::{
    ast::{Expr, Literal},
    lexer::{Token, TokenType},
    reporting::ErrorReporter,
    WmdError,
};

/// Check if next token matches pattern.
/// Implemented as macro so pattern matching syntax can be used
macro_rules! check {
    ($self:ident, $token_typ:pat) => {
        if $self.is_at_end() {
            false
        } else {
            matches!($self.peek().typ, $token_typ)
        }
    };
}

/// Consume token if it matches pattern.
/// Implemented as macro so pattern matching syntax can be used
macro_rules! match_tok {
    ($self:ident, $token_typ:pat) => {
        if check!($self, $token_typ) {
            $self.advance();
            true
        } else {
            false
        }
    };
}

pub struct Parser<'source, R> {
    tokens: Vec<Token<'source>>,
    current: usize,
    reporter: R,
}

impl<'source, R: ErrorReporter> Parser<'source, R> {
    pub fn new(tokens: Vec<Token<'source>>, reporter: R) -> Self {
        Self {
            tokens,
            current: 0,
            reporter,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, WmdError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, WmdError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.comparison()?;

        while match_tok!(self, TokenType::BangEqual | TokenType::EqualEqual) {
            let operator = self.previous().try_into()?;
            let right = self.comparison()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.term()?;

        while match_tok!(
            self,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let operator = self.previous().try_into()?;
            let right = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.factor()?;

        while match_tok!(self, TokenType::Minus | TokenType::Plus) {
            let operator = self.previous().try_into()?;
            let right = self.factor()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.unary()?;

        while match_tok!(self, TokenType::Slash | TokenType::Star) {
            let operator = self.previous().try_into()?;
            let right = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, WmdError> {
        if match_tok!(self, TokenType::Bang | TokenType::Minus) {
            let operator = self.previous().try_into()?;
            let right = self.unary()?;

            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, WmdError> {
        if match_tok!(self, TokenType::False) {
            Ok(Expr::Literal(Literal::Bool(false)))
        } else if match_tok!(self, TokenType::True) {
            Ok(Expr::Literal(Literal::Bool(true)))
        } else if match_tok!(self, TokenType::Nil) {
            Ok(Expr::Literal(Literal::Nil))
        } else if match_tok!(
            self,
            TokenType::Number | TokenType::Quantity | TokenType::String
        ) {
            let literal = self.previous_mut().literal.take().unwrap();

            Ok(Expr::Literal(literal.into()))
        } else if match_tok!(self, TokenType::LBracket) {
            let mut exprs = Vec::new();

            // Allows trailing comma in list
            while !check!(self, TokenType::RBracket) && !self.is_at_end() {
                exprs.push(self.expression()?);

                // Consume ',' if another element is in list
                if !check!(self, TokenType::RBracket) {
                    self.consume(TokenType::Comma, "Expect ',' between list elements.")?;
                }
            }

            self.consume(TokenType::RBracket, "Expect ']' after list.")?;
            Ok(Expr::List(exprs))
        } else if match_tok!(self, TokenType::LParen) {
            let expr = self.expression()?;

            self.consume(TokenType::RParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
    }

    fn advance(&mut self) -> &Token<'source> {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }

    fn peek(&self) -> &Token<'source> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token<'source> {
        &self.tokens[self.current - 1]
    }

    /// Mutable get previous token literal so it can be taken without cloning
    fn previous_mut(&mut self) -> &mut Token<'source> {
        &mut self.tokens[self.current - 1]
    }

    fn consume(&mut self, typ: TokenType, msg: &str) -> Result<&Token, WmdError> {
        if check!(self, typ) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek(), msg))
        }
    }

    fn error(&self, token: &Token, msg: &str) -> WmdError {
        self.reporter.error_token(token, msg);
        WmdError::ParseError
    }

    /// Discards tokens until statement boundary is found
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().typ == TokenType::SemiColon {
                return;
            }

            match self.peek().typ {
                TokenType::Fn | TokenType::For | TokenType::If | TokenType::While => return,
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let content = "true";
        panic!()
    }
}
