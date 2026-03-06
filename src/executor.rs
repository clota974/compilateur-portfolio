use std::collections::HashMap;
use crate::ast::Ast;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::var_env::{VarEnv, VarEnvError, VarEnvOk, VarEnvResult, VarValue};
use crate::visitors::{ExprEvaluator, ExprVisitor};

pub struct Executor {
    ast: Ast,
    env: HashMap<String, VarValue>,
}
impl Executor {
    pub fn new(ast: Ast) -> Self {
        Self {
            ast,
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let iter = self.ast.stmt.iter();

        for stmt in iter {
            let s = stmt.clone();
            match stmt {
                Stmt::VarDecl(s) => {
                    let value: VarValue = self.evaluate(&s.init_value);
                    let mut s = &mut self;
                    //.define(&s.token.lexeme, value);
                },
                Stmt::Return(s) => {
                    let value = self.evaluate(&s.expr);
                    println!("Return : {}", value.display());
                },
                _ => {
                    ()
                }
            }
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> VarValue {
        let evaluator = ExprEvaluator {
            values: self.env.clone()
        };

        expr.accept(&evaluator)
    }

    pub fn define(&mut self, name: &str, value: VarValue) -> Result<(), VarEnvError> {
        if self.values.contains_key(name) {
            return Err(VarEnvError::new(name, "Attempted to initialize an exisiting variable."))
        }
        self.values.insert(name.to_string(), value.clone());
        Ok(())
    }

    pub fn get(&mut self, name: &str) -> VarEnvResult {
        if !self.values.contains_key(name) {
            return Err(VarEnvError::new(name, "Attempted to read unknown variable"))
        }
        let value = self.values.get(name).unwrap();
        Ok(VarEnvOk::new(name, value.to_owned()))
    }

    pub fn assign(&mut self, name: &str, value: VarValue) -> Result<(), VarEnvError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            return Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value)?;
            return Ok(())
        }

        Err(VarEnvError::new(name, "Attempted to assign an unknown variable."))
    }
}

