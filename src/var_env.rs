use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct VarEnvError {
    name: String,
    error: String,
}

impl VarEnvError {
    pub fn new(name: &str, error: &str) -> VarEnvError {
        VarEnvError {
            name: name.to_string(),
            error: error.to_string(),
        }
    }
}

pub struct VarEnvOk {
    pub name: String,
    pub value: VarValue,
}
impl VarEnvOk {
    pub fn new(name: &str, value: VarValue) -> VarEnvOk {
        VarEnvOk {
            name: name.to_string(),
            value,
        }
    }
}

pub type VarEnvResult = Result<VarEnvOk, VarEnvError>;

#[derive(Clone)]
pub enum VarValue {
    String(String),
    Number(f64),
}
impl VarValue {
    pub fn display(&self) -> String {
        match self {
            VarValue::String(s) => s.to_owned(),
            VarValue::Number(n) => format!("{}", n),
        }
    }
}

pub struct VarEnv {
    values: HashMap<String, VarValue>,
    parent: Option<Box<VarEnv>>,
}
impl VarEnv {
    pub fn new() -> VarEnv {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }
}
