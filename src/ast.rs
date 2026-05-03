use crate::printer::Printer;
use crate::stmt::Stmt;
use crate::visitors::stmt_debugger::StmtDebugger;
use crate::visitors::stmt_visitor::StmtVisitor;

pub struct Ast {
    pub stmt: Vec<Stmt>,
}
impl Ast {
    pub fn new(stmt: Vec<Stmt>) -> Ast {
        Ast { stmt }
    }
    pub fn accept<V: StmtVisitor + Copy>(&self, visitor: V) -> Vec<V::Output> {
        let stmt_iter = self.stmt.iter();
        let mut output: Vec<V::Output> = Vec::new();

        for stmt in stmt_iter {
            output.push(stmt.accept(visitor))
        }
        output
    }
    pub fn print(self) {
        let output = self.accept(StmtDebugger);
        for o in output.iter() {
            println!("{}", o);
        }
    }

    pub fn debug(&self) {
        self.accept(Printer);
    }
}
