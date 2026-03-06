mod errors;
mod parser;
mod scanner;
mod token_types;
mod lexeme_maker;
mod scanbuf;
mod expr;
mod stmt;
mod printer;
mod ast;
mod formatter;
mod visitors;
mod var_env;
mod executor;

use crate::parser::{generate_ast, print_if_ok};
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
    let result = generate_ast(tokens);
    result.ast.debug();

    /*
    println!("\n------ Evaluation -------\n");
    let output = &parsing.ast.unwrap().print();
    println!("Output : {}", output);
     */
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

    #[test]
    fn test_priority() {
        return;
        let o1 = print_ast_from("36 * 4 + (5 * 2.4 + 6)");
        assert_eq!(o1, "(+ (* 36 4) [ (+ (* 5 2.4) 6) ])");

        let o2 = print_ast_from("36 * 4 + 5 * 2.4 + 6");
        assert_eq!(o2, "(+ (+ (* 36 4) (* 5 2.4)) 6)");
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
}
