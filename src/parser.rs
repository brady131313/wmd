use crate::{
    ast::{Expr, Literal, Stmt},
    lexer::{Token, TokenType},
    reporting::ErrorReporter,
    WmdError,
};

/// Consume token if it matches pattern.
/// Implemented as macro so pattern matching syntax can be used
macro_rules! match_tok {
    ($self:ident, $token_typ:expr) => {
        if $self.check($token_typ) {
            $self.advance();
            true
        } else {
            false
        }
    };
    ($self:ident, $head_typ:expr, $($tail_typ:expr),+) => {
        match_tok!($self, $head_typ) || match_tok!($self, $($tail_typ),+)
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, WmdError> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, WmdError> {
        let decl = if match_tok!(self, TokenType::Let) {
            self.let_declaration()
        } else {
            self.statement()
        };

        match decl {
            Ok(d) => Ok(d),
            Err(_) => {
                self.synchronize();
                Ok(Stmt::None)
            }
        }
    }

    fn let_declaration(&mut self) -> Result<Stmt, WmdError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .try_into()?;

        let mut inititializer = Expr::Literal(Literal::Nil);
        if match_tok!(self, TokenType::Equal) {
            inititializer = self.expression()?;
        }

        self.consume(TokenType::SemiColon, "Expect ';' after let declaration.")?;
        Ok(Stmt::Let(name, inititializer))
    }

    fn statement(&mut self) -> Result<Stmt, WmdError> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Stmt, WmdError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, WmdError> {
        if match_tok!(self, TokenType::LBrace) {
            self.block()
        } else {
            self.or()
        }
    }

    /// Assumes that the opening '{' has already been consumed.
    /// Consumes declarations but permits the last statement to exclude
    /// a semicolon if it is an expression. If semicolon is still included
    /// than implicit nil expr is added to block
    fn block(&mut self) -> Result<Expr, WmdError> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(s) => stmts.push(s),
                Err(e) => todo!(),
            }
        }

        self.consume(TokenType::RBrace, "Expect '}' after block.")?;
        todo!()
        // Ok(Expr::Block(stmts))
    }

    fn or(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.and()?;

        while match_tok!(self, TokenType::Or) {
            let operator = self.previous().try_into()?;
            let right = self.and()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.equality()?;

        while match_tok!(self, TokenType::And) {
            let operator = self.previous().try_into()?;
            let right = self.equality()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.comparison()?;

        while match_tok!(self, TokenType::BangEqual, TokenType::EqualEqual) {
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
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator = self.previous().try_into()?;
            let right = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.factor()?;

        while match_tok!(self, TokenType::Minus, TokenType::Plus) {
            let operator = self.previous().try_into()?;
            let right = self.factor()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, WmdError> {
        let mut expr = self.unary()?;

        while match_tok!(self, TokenType::Slash, TokenType::Star) {
            let operator = self.previous().try_into()?;
            let right = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, WmdError> {
        if match_tok!(self, TokenType::Bang, TokenType::Minus) {
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
            TokenType::Number,
            TokenType::Quantity,
            TokenType::String
        ) {
            let literal = self.previous_mut().literal.take().unwrap();

            Ok(Expr::Literal(literal.into()))
        } else if match_tok!(self, TokenType::Identifier) {
            let ident = self.previous().try_into()?;
            Ok(Expr::Var(ident))
        } else if match_tok!(self, TokenType::LBracket) {
            let mut exprs = Vec::new();

            // Allows trailing comma in list
            while !self.check(TokenType::RBracket) && !self.is_at_end() {
                exprs.push(self.expression()?);

                // Consume ',' if another element is in list
                if !self.check(TokenType::RBracket) {
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

    fn check(&self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().typ == typ
        }
    }

    fn consume(&mut self, typ: TokenType, msg: &str) -> Result<&Token, WmdError> {
        if self.check(typ) {
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
