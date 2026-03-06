use crate::token_types::{Token, TokenKind};
use std::iter::Peekable;
use std::string::ToString;
use std::vec::IntoIter;

pub enum Expr {
    Literal(f64),
    Binary {
        left: Option<Box<Expr>>,
        operator: Token,
        right: Option<Box<Expr>>,
    },
    Grouping(Box<Expr>),
}

pub struct ParseError {
    line: usize,
    lexeme: String,
    message: String,
}
impl ParseError {
    pub fn from_token(token: &Token, message: &str) -> Self {
        Self {
            line: token.line,
            lexeme: token.lexeme.to_string(),
            message: message.to_string(),
        }
    }
}

pub struct ParseResult {
    pub ast: Option<Box<Expr>>,
    pub errors: Vec<ParseError>,
    pub had_errors: bool,
}
impl ParseResult {
    fn new(expr: Option<Box<Expr>>) -> ParseResult {
        ParseResult {
            ast: expr,
            errors: Vec::new(),
            had_errors: false,
        }
    }

    fn add_errors(&mut self, errors: &mut Vec<ParseError>) {
        self.had_errors = true;
        self.errors.append(errors)
    }
}

struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    errors: Vec<ParseError>,
    had_error: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            had_error: false,
            errors: Vec::new(),
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn add_error(&mut self, err: ParseError) {
        self.had_error = true;
        self.errors.push(err);
    }

    pub fn print_errors(&self) {
        if !self.had_error  {
            eprintln!("Parsing completed successfully!");
            return ;
        }
        eprintln!("Following errors have been raised by parser : ");

        for err in self.errors.iter() {
            eprintln!(
                "    ParseError -> Line {} -> Input : {} -> {}",
                err.line, err.lexeme, err.message
            )
        }
    }

    fn peek(&mut self) -> &Token {
        static EOF: Token = Token {
            line: 0,
            column: 0,
            lexeme: String::new(),
            kind: TokenKind::EOF,
        };
        self.tokens.peek().unwrap_or(&EOF)
    }

    fn next(&mut self) -> Token {
        let token = self.tokens.next().unwrap_or_else(|| Token {
            line: 0,
            column: 0,
            lexeme: "<EOF>".to_string(),
            kind: TokenKind::EOF,
        });
        log::debug!(" > {}", token.lexeme);
        token
    }

    pub fn parse_ast(&mut self) -> ParseResult {
        let result = self.parse_expr();
        let token = self.next();
        if token.kind != TokenKind::EOF {
            self.add_error(ParseError::from_token(
                &token,
                "Unexpected token after expression",
            ));
        }

        let mut result = ParseResult::new(result);
        if self.had_error  {
            self.print_errors();
            println!("---> Parsing FAILED ");
            result.add_errors(&mut self.errors);
        }
        result
    }

    fn parse_expr(&mut self) -> Option<Box<Expr>> {
        let mut left = self.parse_term();

        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let operator = self.next();
            let right = self.parse_term();

            left = Some(Box::new(Expr::Binary {
                left,
                operator,
                right,
            }));
        }
        left
    }

    fn parse_term(&mut self) -> Option<Box<Expr>> {
        let mut left = self.parse_factor();

        while matches!(self.peek().kind, TokenKind::Asterisk | TokenKind::Slash) {
            let operator = self.next();
            let right = self.parse_factor();

            left = Some(Box::new(Expr::Binary {
                left,
                operator,
                right,
            }));
        }
        left
    }
    fn parse_factor(&mut self) -> Option<Box<Expr>> {
        let token = self.next();

        match token.kind {
            TokenKind::Number => {
                let number: Result<f64, _> = token.lexeme.parse();
                match number {
                    Ok(number) => Some(Box::new(Expr::Literal(number))),
                    _ => {
                        self.add_error(ParseError::from_token(&token, "Failed to parse as number"));
                        None
                    }
                }
            }
            TokenKind::ParOpen => {
                let expr = self.parse_expr();

                let token = self.next();
                if token.kind != TokenKind::ParClose {
                    self.add_error(ParseError::from_token(&token, "Unclosed parenthesis found"));
                }
                Some(Box::new(Expr::Grouping(expr?)))
            }
            _ => {
                self.add_error(ParseError::from_token(&token, "Syntax error"));
                None
            }
        }
    }
}

pub fn generate_ast(tokens: Vec<Token>) -> ParseResult {
    let mut parser = Parser::new(tokens);
    parser.parse_ast()
}

fn literal(number: &f64) -> String {
    format!("{}", number)
}

fn binary(left: &Option<Box<Expr>>, operator: &Token, right: &Option<Box<Expr>>) -> String {
    let lprint = print_if_ok(left);
    let rprint = print_if_ok(right);

    format!("({} {} {})", operator.lexeme, lprint, rprint)
}

pub fn print_if_ok(ast: &Option<Box<Expr>>) -> String {
    match ast {
        Some(expr) => print_expr(expr),
        _ => "<?>".to_string(),
    }
}

pub fn print_expr(ast: &Expr) -> String {
    let mut output = String::new();
    let buf = match ast {
        Expr::Literal(number) => literal(number),
        Expr::Binary {
            left,
            operator,
            right,
        } => binary(left, operator, right),
        Expr::Grouping(grp) => format!("[ {} ]", print_expr(grp)),
    };
    output.push_str(&buf);
    output
}
