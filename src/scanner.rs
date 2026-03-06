use crate::errors::CompilError;
use crate::token_types::{PartialToken, Token, TokenKind};
use crate::lexeme_maker::LexemeMaker;
use crate::scanbuf::ScanBuf;

pub enum ScanResult {
    Skip,
    Token(PartialToken),
    Error(String),
}

pub struct Scanner {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Scanner { chars: input.chars().collect(), pos: 0, line: 0, column: 0, tokens: Vec::new() }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.tokens = Vec::new();
        self.line = 1;
        self.column = 1;

        while let Some(c) = self.peek() {
            let result: ScanResult = match c {
                '\n' => {
                    self.newline();
                    self.next();
                    ScanResult::Skip
                },
                ';' => LexemeMaker::make_semicolon(self),
                '"' => LexemeMaker::make_string_chain(self),
                '0'..='9' => LexemeMaker::make_number(self),
                'a'..='z' | 'A'..='Z' => LexemeMaker::make_alphanum(self),
                '\r' | '\t' | ' '  => {
                    self.next();
                    ScanResult::Skip
                }
                _ => LexemeMaker::make_operator(self),
            };

            if let ScanResult::Token(token) = result {
                self.push_token(token);
            } else if let ScanResult::Error(err) = result {
                CompilError::raise_now(err, self.line);
            }
        }

        self.tokens.push(Token {
            line: self.line + 1,
            column: 0,
            kind: TokenKind::EOF,
            lexeme: String::new(),
        });

        self.tokens.clone()
    }

    fn push_token(&mut self, p_token: PartialToken) {
        let start_col = self.column - p_token.lexeme.chars().count();
        self.tokens.push(Token {
            line: self.line,
            column: start_col,
            kind: p_token.kind,
            lexeme: p_token.lexeme
        })
    }

    fn newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub fn next(&mut self) -> &mut Self {
        let c = self.peek();
        if c.is_some() { self.pos += 1; self.column += 1; }
        self
    }

    pub fn next_if(&mut self, matches: &str) -> Option<char> {
        let c = match self.peek() {
            Some(c) => c,
            _ => return None
        };
        if c.to_string() == matches {
            self.next().peek()
        } else {
            None
        }
    }

    pub fn next_until(&mut self, matches: char) -> String {
        let mut buffer = ScanBuf::new();
        while let Some(c) = self.next().peek() {
            buffer.push(c);
            if (c == matches) {
                return buffer.to_str()
            }
        };
        unreachable!();
    }

    pub fn move_to_buffer(&mut self, buffer: &mut ScanBuf) {
        let c = self.peek().unwrap();
        buffer.push(c);
        self.next();
    }
}
