use crate::executor::Executor;
use crate::visitors::{ExprEvaluator, StmtVisitor};
use crate::stmt::{ExprStmt, Return, VarDecl};

#[derive(Copy, Clone)]
pub struct Printer;

impl StmtVisitor for Printer {
    type Output = ();
    fn visit_vardecl(&self, stmt: &VarDecl) {
        let expr_value = stmt.init_value.accept(Executor);
        println!("[vardecl] Declared {} to {}", stmt.token.lexeme, expr_value)
    }

    fn visit_expr(&self, stmt: &ExprStmt) {
        let expr_value = stmt.expr.accept(Executor);
        println!("[expr] Value : {}", expr_value);
    }

    fn visit_return(&self, stmt: &Return) {
        let expr_value = stmt.expr.accept(Executor);
        println!("[return] Value : {}", expr_value);
    }
}