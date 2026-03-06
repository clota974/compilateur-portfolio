use crate::expr::Expr;
use crate::var_env::VarEnv;

pub trait ExprVisitor {
    type Output;

    fn visit_number(&self, value: f64) -> Self::Output;

    fn visit_identifier(&self, name: &str) -> Self::Output;
    fn visit_add(&self, left: &Expr, right: &Expr) -> Self::Output;

    fn visit_mul(&self, left: &Expr, right: &Expr) -> Self::Output;
    fn visit_sub(&self, left: &Expr, right: &Expr) -> Self::Output;
    fn visit_div(&self, left: &Expr, right: &Expr) -> Self::Output;
}