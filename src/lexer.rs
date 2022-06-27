use logos::{Lexer, Logos};

use crate::ast::{Quantity, TimeUnit, Unit};

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[regex(r"([+-]?([0-9]+\.?[0-9]*|\.[0-9]+))([xsm%])", lex_quantity)]
    Quantity(Quantity),
    #[regex(r"[+-]?([0-9]+\.?[0-9]*|\.[0-9]+)", lex_number)]
    Num(f64),
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex("true|false", lex_bool)]
    Bool(bool),
    #[token("!")]
    OpBang,
    #[token("=")]
    OpAssign,
    #[token("+")]
    OpPlus,
    #[token("-")]
    OpMinus,
    #[token("/")]
    OpSlash,
    #[token("*")]
    OpStar,
    #[token("!=")]
    OpNotEq,
    #[token("==")]
    OpEq,
    #[token("<=")]
    OpLte,
    #[token("<")]
    OpLt,
    #[token(">=")]
    OpGte,
    #[token(">")]
    OpGt,
    #[token("(")]
    LRound,
    #[token(")")]
    RRound,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token(",")]
    Comma,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("nil")]
    Nil,
    #[regex(r"(\n)")]
    Newline,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("let")]
    Let,
    #[token("while")]
    While,
    EOI,
    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

fn lex_bool(lex: &mut Lexer<Token>) -> bool {
    match lex.slice() {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    }
}

fn lex_number(lex: &mut Lexer<Token>) -> f64 {
    let slice = lex.slice();
    slice.parse().unwrap()
}

fn lex_quantity(lex: &mut Lexer<Token>) -> Quantity {
    let slice = lex.slice();
    let num: f64 = slice[..slice.len() - 1].parse().unwrap();

    let unit = match &slice[slice.len() - 1..slice.len()] {
        "x" => Unit::Rep,
        "%" => Unit::Percent,
        "s" => Unit::Time(TimeUnit::Second),
        "m" => Unit::Time(TimeUnit::Minute),
        _ => unreachable!(),
    };

    Quantity::new(num, unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let wmd_content = r#"3x rpe(8) 30.5s "this a string" fn if else true
            nil 5.0 10 false + - / * != ! == > >= < <= = and or
            
            let"#;

        let tokens: Vec<_> = Token::lexer(&wmd_content).collect();
        assert_eq!(
            tokens,
            &[
                Token::Quantity(Quantity::new(3.0, Unit::Rep)),
                Token::Ident,
                Token::LRound,
                Token::Num(8.0),
                Token::RRound,
                Token::Quantity(Quantity::new(30.5, Unit::Time(TimeUnit::Second))),
                Token::String,
                Token::Fn,
                Token::If,
                Token::Else,
                Token::Bool(true),
                Token::Newline,
                Token::Nil,
                Token::Num(5.0),
                Token::Num(10.0),
                Token::Bool(false),
                Token::OpPlus,
                Token::OpMinus,
                Token::OpSlash,
                Token::OpStar,
                Token::OpNotEq,
                Token::OpBang,
                Token::OpEq,
                Token::OpGt,
                Token::OpGte,
                Token::OpLt,
                Token::OpLte,
                Token::OpAssign,
                Token::And,
                Token::Or,
                Token::Newline,
                Token::Newline,
                Token::Let
            ]
        );
    }
}
