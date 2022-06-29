use crate::{
    ast::{Expr, Literal},
    lexer::{Token, TokenType},
    WmdError,
};

macro_rules! check {
    ($self:ident, $token_typ:pat) => {
        if $self.is_at_end() {
            false
        } else {
            matches!($self.peek().typ, $token_typ)
        }
    };
}

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

pub struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    current: usize,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, WmdError> {
        self.primary()
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
            let literal = self.previous().literal.clone().unwrap();
            Ok(Expr::Literal(literal.into()))
        } else {
            todo!()
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
