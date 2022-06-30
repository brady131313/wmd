use crate::{
    ast::{BinaryOp, Expr, ExprVisitor, Literal, OpToken, UnaryOp},
    WmdError,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Literal, WmdError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal(&mut self, value: &Literal) -> Result<Literal, WmdError> {
        Ok(value.clone())
    }

    fn visit_group(&mut self, expr: &Expr) -> Result<Literal, WmdError> {
        self.evaluate(expr)
    }

    fn visit_list(&mut self, exprs: &[Expr]) -> Result<Literal, WmdError> {
        todo!()
    }

    fn visit_binary(
        &mut self,
        lhs: &Expr,
        op: OpToken<BinaryOp>,
        rhs: &Expr,
    ) -> Result<Literal, WmdError> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        match op.typ {
            BinaryOp::Plus => add_or_concat(lhs, op, rhs),
            BinaryOp::Minus => binary_num_op(lhs, op, rhs, |a, b| Literal::Number(a - b)),
            BinaryOp::Slash => binary_num_op(lhs, op, rhs, |a, b| Literal::Number(a / b)),
            BinaryOp::Star => binary_num_op(lhs, op, rhs, |a, b| Literal::Number(a * b)),
            BinaryOp::Less => binary_num_op(lhs, op, rhs, |a, b| Literal::Bool(a < b)),
            BinaryOp::LessEqual => binary_num_op(lhs, op, rhs, |a, b| Literal::Bool(a <= b)),
            BinaryOp::Greater => binary_num_op(lhs, op, rhs, |a, b| Literal::Bool(a > b)),
            BinaryOp::GreaterEqual => binary_num_op(lhs, op, rhs, |a, b| Literal::Bool(a >= b)),
            BinaryOp::EqualEqual => Ok(Literal::Bool(lhs == rhs)),
            BinaryOp::BangEqual => Ok(Literal::Bool(lhs != rhs)),
        }
    }

    fn visit_unary(&mut self, op: OpToken<UnaryOp>, expr: &Expr) -> Result<Literal, WmdError> {
        let rhs = self.evaluate(expr)?;

        match op.typ {
            UnaryOp::Minus => {
                let num = rhs.as_number().ok_or(WmdError::UnaryNumberRequired(op))?;
                Ok(Literal::Number(-num))
            }
            UnaryOp::Bang => Ok(Literal::Bool(!rhs.is_truthy())),
        }
    }
}

fn binary_num_op<F>(
    lhs: Literal,
    op: OpToken<BinaryOp>,
    rhs: Literal,
    f: F,
) -> Result<Literal, WmdError>
where
    F: Fn(f64, f64) -> Literal,
{
    let lhs = lhs.as_number().ok_or(WmdError::BinaryNumberRequired(op))?;
    let rhs = rhs.as_number().ok_or(WmdError::BinaryNumberRequired(op))?;
    Ok(f(lhs, rhs))
}

fn add_or_concat(lhs: Literal, op: OpToken<BinaryOp>, rhs: Literal) -> Result<Literal, WmdError> {
    match (lhs, rhs) {
        (Literal::String(lhs), Literal::String(rhs)) => Ok(Literal::String(format!("{lhs}{rhs}"))),
        (Literal::String(lhs), rhs) => Ok(Literal::String(format!("{lhs}{rhs}"))),
        (lhs, Literal::String(rhs)) => Ok(Literal::String(format!("{lhs}{rhs}"))),
        (lhs, rhs) => {
            let lhs = lhs
                .as_number()
                .ok_or(WmdError::NumberOrStringRequired(op))?;

            let rhs = rhs
                .as_number()
                .ok_or(WmdError::NumberOrStringRequired(op))?;

            Ok(Literal::Number(lhs + rhs))
        }
    }
}
