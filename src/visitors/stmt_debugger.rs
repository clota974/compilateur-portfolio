use crate::visitors::expr_debugger::ExprDebugger;
use crate::visitors::stmt_visitor::StmtVisitor;
use crate::stmt::{ExprStmt, Return, VarDecl};
use crate::var_env::VarEnv;

pub struct StmtDebugger<'a> {
    env: &'a VarEnv,
}
impl StmtVisitor for StmtDebugger<'_> {
    type Output = String;
    fn visit_vardecl(&self, stmt: &VarDecl) -> String {
        let lexeme = &stmt.token.lexeme;
        let value = stmt.init_value.accept(&ExprDebugger);
        format!("decl({}, {})", lexeme, value)
    }

    fn visit_expr(&self, expr: &ExprStmt) -> String {
        format!("exprStmt({})", expr.expr.accept(&ExprDebugger))
    }

    fn visit_return(&self, expr: &Return) -> String {
        format!("return({})", expr.expr.accept(&ExprDebugger))
    }
}