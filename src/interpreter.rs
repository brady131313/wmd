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
        todo!()
    }

    fn visit_unary(&mut self, op: OpToken<UnaryOp>, expr: &Expr) -> Result<Literal, WmdError> {
        let rhs = self.evaluate(expr)?;

        match op.typ {
            UnaryOp::Minus => {
                let num = rhs.as_number().ok_or(WmdError::UnaryNumberRequired(op))?;
                Ok(Literal::Number(-num))
            }
            UnaryOp::Bang => todo!(),
        }
    }
}
