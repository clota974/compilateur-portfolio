use crate::expr::Expr;
use crate::visitors::StmtVisitor;
use crate::token_types::Token;
use crate::var_env::VarValue;

pub enum Stmt {
    ExprStmt(ExprStmt),
    VarDecl(VarDecl),
    Return(Return),
    Call(Call)
}
impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: V) -> V::Output {
        match self {
            Stmt::VarDecl(stmt) => visitor.visit_vardecl(stmt),
            Stmt::ExprStmt(stmt) => visitor.visit_expr(stmt),
            Stmt::Return(stmt) => visitor.visit_return(stmt),
            _ => panic!("Can't visit unknown statement")
        }
    }
}

pub struct ExprStmt {
    pub expr: Expr
}
pub struct Block {
    pub body: Vec<Expr>
}

pub struct VarDecl {
    pub token: Token,
    pub init_value: Expr
}

pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>
}

pub struct Return {
    pub expr: Expr,
}