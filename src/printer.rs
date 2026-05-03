use crate::expr::Expr;
use crate::stmt::{ExprStmt, Return, VarDecl};
use crate::visitors::{ExprVisitor, StmtVisitor};

#[derive(Copy, Clone)]
pub struct Printer;

impl StmtVisitor for Printer {
    type Output = ();

    fn visit_vardecl(&self, stmt: &VarDecl) {
        println!(
            "[vardecl] {} = {}",
            stmt.token.lexeme,
            stmt.init_value.accept(self)
        );
    }
    fn visit_expr(&self, stmt: &ExprStmt) {
        println!("[expr] {}", stmt.expr.accept(self));
    }
    fn visit_return(&self, stmt: &Return) {
        println!("[return] {}", stmt.expr.accept(self));
    }
}

impl ExprVisitor for Printer {
    type Output = String;

    fn visit_number(&self, value: f64) -> String {
        value.to_string()
    }
    fn visit_identifier(&self, name: &str) -> String {
        name.to_string()
    }
    fn visit_add(&self, left: &Expr, right: &Expr) -> String {
        format!("({} + {})", left.accept(self), right.accept(self))
    }
    fn visit_mul(&self, left: &Expr, right: &Expr) -> String {
        format!("({} * {})", left.accept(self), right.accept(self))
    }
    fn visit_sub(&self, left: &Expr, right: &Expr) -> String {
        format!("({} - {})", left.accept(self), right.accept(self))
    }
    fn visit_div(&self, left: &Expr, right: &Expr) -> String {
        format!("({} / {})", left.accept(self), right.accept(self))
    }
}
