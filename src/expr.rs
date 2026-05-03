use crate::executor::Executor;
use crate::token_types::{Token, TokenKind};
use crate::var_env::VarEnv;
use crate::visitors::ExprVisitor;

#[derive(Clone)]
pub enum Expr {
    Literal(f64),
    Identifier(String),
    Binary {
        left: Option<Box<Expr>>,
        operator: Token,
        right: Option<Box<Expr>>,
    },
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Output {
        match self {
            Expr::Literal(n) => visitor.visit_number(*n as f64),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.as_deref().unwrap();
                let right = right.as_deref().unwrap();
                match operator.kind {
                    TokenKind::Plus => visitor.visit_add(left, right),
                    TokenKind::Minus => visitor.visit_sub(left, right),
                    TokenKind::Asterisk => visitor.visit_mul(left, right),
                    TokenKind::Slash => visitor.visit_div(left, right),
                    _ => panic!("Unknown operator"),
                }
            }
            Expr::Grouping(inner) => inner.accept(visitor),
            Expr::Identifier(i) => visitor.visit_identifier(i),
        }
    }
}
