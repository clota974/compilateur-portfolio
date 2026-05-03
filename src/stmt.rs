use crate::expr::Expr;
use crate::token_types::Token;
use crate::var_env::VarValue;
use crate::visitors::StmtVisitor;

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    VarDecl(VarDecl),
    Return(Return),
    Call(Call),
}
impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: V) -> V::Output {
        match self {
            Stmt::VarDecl(stmt) => visitor.visit_vardecl(stmt),
            Stmt::ExprStmt(stmt) => visitor.visit_expr(stmt),
            Stmt::Return(stmt) => visitor.visit_return(stmt),
            _ => panic!("Can't visit unknown statement"),
        }
    }
}

#[derive(Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}
pub struct Block {
    pub body: Vec<Expr>,
}

#[derive(Clone)]
pub struct VarDecl {
    pub token: Token,
    pub init_value: Expr,
}

#[derive(Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Clone)]
pub struct Return {
    pub expr: Expr,
}
