mod errors;
mod parser;
mod scanner;
mod token_types;
mod lexeme_maker;
mod scanbuf;

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
            "Type : {:?}, Lexeme: {}, Line: {}",
            t.kind, t.lexeme, t.line
        )
    }

    println!("\n------ Parser -------\n");
    let parsing = generate_ast(tokens);
    let print = print_if_ok(&parsing.ast);
    println!("\n\n{}", print)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{ParseError, ParseResult};

    fn ast_from(input: &str) -> ParseResult {
        let mut scanner = Scanner::new(&input);
        let tokens = scanner.tokenize();
        generate_ast(tokens)
    }

    fn print_ast_from(input: &str) -> String {
        let ast = ast_from(input).ast.unwrap();
        parser::print_expr(&ast)
    }

    #[test]
    fn test_priority() {
        let o1 = print_ast_from("36 * 4 + (5 * 2.4 + 6)");
        assert_eq!(o1, "(+ (* 36 4) [ (+ (* 5 2.4) 6) ])");

        let o2 = print_ast_from("36 * 4 + 5 * 2.4 + 6");
        assert_eq!(o2, "(+ (+ (* 36 4) (* 5 2.4)) 6)");
    }

    #[test]
    fn test_parse_errors() {
        let o1 = ast_from("36 * 4 + 5 * 2.4 + 6)");
        let o2 = ast_from("36 * 4 (+ 5 * 2.4 + 6)");
        let o3 = ast_from("36 * 4 (+ 5 * 2.4 + 6");
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
