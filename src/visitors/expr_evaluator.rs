use std::collections::HashMap;
use crate::visitors::ExprVisitor;
use crate::expr::Expr;
use crate::var_env::{VarEnv, VarEnvOk, VarValue};
use crate::var_env::VarValue::Number;

pub fn accept_for_number(ctx: &ExprEvaluator, left: &Expr) -> f64 {
    let value = left.accept(ctx);
    match value {
        Number(v) => v,
        _ => panic!("Expected a number")
    }
}

pub struct ExprEvaluator {
    pub values: HashMap<String, VarValue>
}

impl ExprVisitor for ExprEvaluator {
    type Output = VarValue;

    fn visit_number(&self, value: f64) -> Self::Output {
        VarValue::Number(value)
    }
    fn visit_identifier(&self, name: &str) -> Self::Output {
        let result = self.values.get(name).unwrap();
        result.to_owned()
    }


    fn visit_add(&self, left: &Expr, right: &Expr) -> Self::Output {
        let l = accept_for_number(self, left);
        let r = accept_for_number(self, right);
        VarValue::Number(l + r)
    }

    fn visit_mul(&self, left: &Expr, right: &Expr) -> Self::Output {
        let l = accept_for_number(self, left);
        let r = accept_for_number(self, right);
        VarValue::Number(l * r)
    }

    fn visit_sub(&self, left: &Expr, right: &Expr) -> Self::Output {
        let l = accept_for_number(self, left);
        let r = accept_for_number(self, right);
        VarValue::Number(l - r)
    }

    fn visit_div(&self, left: &Expr, right: &Expr) -> Self::Output {
        let l = accept_for_number(self, left);
        let r = accept_for_number(self, right);
        if r == 0f64 {
            panic!("Attempt to divide by ero")
        }
        VarValue::Number(l / r)
    }
}
