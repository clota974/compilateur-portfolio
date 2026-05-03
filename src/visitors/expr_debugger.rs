use crate::expr::Expr;
use crate::var_env::{VarEnv, VarEnvResult};
use crate::visitors::expr_visitor::ExprVisitor;
use std::fmt::format;

#[derive(Copy, Clone)]
pub struct ExprDebugger;
impl ExprVisitor for ExprDebugger {
    type Output = String;

    fn visit_number(&self, value: f64) -> Self::Output {
        format!("{}", value)
    }

    fn visit_identifier(&self, name: &str) -> Self::Output {
        format!("id({})", name)
    }

    fn visit_add(&self, left: &Expr, right: &Expr) -> Self::Output {
        format!("({} + {})", left.accept(self), right.accept(self))
    }

    fn visit_mul(&self, left: &Expr, right: &Expr) -> Self::Output {
        format!("({} * {})", left.accept(self), right.accept(self))
    }

    fn visit_sub(&self, left: &Expr, right: &Expr) -> Self::Output {
        format!("({} - {})", left.accept(self), right.accept(self))
    }

    fn visit_div(&self, left: &Expr, right: &Expr) -> Self::Output {
        format!("({} / {})", left.accept(self), right.accept(self))
    }
}
