use crate::expr::Expr;
use crate::stmt::{ExprStmt, Return, Stmt, VarDecl};

pub trait StmtVisitor {
    type Output;
    fn visit_vardecl(&self, stmt: &VarDecl) -> Self::Output;
    fn visit_expr(&self, expr: &ExprStmt) -> Self::Output;

    fn visit_return(&self, expr: &Return) -> Self::Output;
}
