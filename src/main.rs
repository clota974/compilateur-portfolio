mod ast;
mod errors;
mod executor;
mod expr;
mod formatter;
mod lexeme_maker;
mod parser;
mod printer;
mod scanbuf;
mod scanner;
mod stmt;
mod token_types;
mod var_env;
mod visitors;

use crate::executor::Executor;
use crate::parser::generate_ast;
use crate::scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    println!("\n------ Tokenizer -------\n");

    let mut scanner = Scanner::new(&contents);
    let tokens = scanner.tokenize();

    for t in &tokens {
        println!(
            "Type : {:?}, Lexeme: {}, Line: {}, Col: {}",
            t.kind, t.lexeme, t.line, t.column
        )
    }

    println!("\n------ Parser -------\n");
    let parsing = generate_ast(tokens);
    parsing.ast.debug();

    println!("\n------ Executing program -------\n");
    let output = Executor::new(parsing.ast).run();
    println!("End of execution");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{ParseError, ParseResult};
    use crate::visitors::StmtDebugger;

    fn ast_from(input: &str) -> ParseResult {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.tokenize();
        generate_ast(tokens)
    }

    fn print_ast_from(input: &str) -> String {
        let ast = ast_from(input).ast;
        let s = ast.accept(StmtDebugger).join("");
        println!("Test : {}", s);
        s
    }

    fn exec_from_str(input: &str) -> String {
        let parsing = ast_from(input);
        Executor::new(parsing.ast).run()
    }

    #[test]
    fn test_priority() {
        let o1 = print_ast_from("let x = 36 * 4 + (5 * 2.4 + 6);");
        assert_eq!(o1, "decl(x, ((36 * 4) + ((5 * 2.4) + 6)))");

        let o2 = print_ast_from("let y = 36 * 4 + 5 * 2.4 + 6");
        assert_eq!(o2, "decl(y, (((36 * 4) + (5 * 2.4)) + 6))");
    }

    #[test]
    fn test_parse_errors() {
        let o1 = ast_from("let x y = 0;");
        let o2 = ast_from("let x = 5 * 3 +;");
        let o3 = ast_from("let y = 3 * (2 + 5;");
        let arr = vec![o1, o2, o3];
        let mut nb_errors: u8 = 0;
        for output in arr.iter() {
            if (output.had_errors) {
                nb_errors += 1;
            }
        }

        assert_eq!(nb_errors, 3);
    }

    #[test]
    fn test_exec_maths() {
        let output = exec_from_str("let x = 10; let y = 6 + 2; return y + 2*x;");
        assert_eq!(output, "28");
    }
}
