use crate::scanbuf::ScanBuf;
use crate::scanner::{ScanResult, Scanner};
use crate::token_types::{PartialToken, TokenKind};
use crate::token_types::TokenKind::{Identifier, SemiColon};

pub struct LexemeMaker{

}
impl LexemeMaker {
    pub fn make_semicolon(scanner: &mut Scanner) -> ScanResult {
       scanner.next();
        ScanResult::Token(PartialToken {
            kind: SemiColon,
            lexeme: ";".to_string()
        })
    }
    pub fn make_string_chain(scanner: &mut Scanner) -> ScanResult {
        let mut buffer = ScanBuf::new();
        scanner.next(); // Consumes first quote

        while let Some(c) = scanner.peek() {
            // Return if not an escaped quote
            if c == '"' && buffer.buffer.last().copied() != Some('\\') {
                scanner.next();
                return ScanResult::Token(
                    buffer.to_token(TokenKind::StringChain)
                )
            }

            if c == '\n' {
                break;
            }

            scanner.move_to_buffer(&mut buffer);
        }

        ScanResult::Error("Unterminated string chain".to_string())
    }
    pub fn make_number(scanner: &mut Scanner) -> ScanResult {
        let mut buffer = ScanBuf::new();
        let mut floating = false;

        while let Some(c) = scanner.peek() {
            if c.is_ascii_digit() {
                scanner.move_to_buffer(&mut buffer);
            } else if c == '.' {
                if floating {
                    return ScanResult::Error("Unparseable number found".to_string());
                }
                floating = true;
                scanner.move_to_buffer(&mut buffer);
            } else {
                break;
            }
        }

        ScanResult::Token(
            buffer.to_token(TokenKind::Number)
        )
    }

    pub fn make_alphanum(scanner: &mut Scanner) -> ScanResult {
        let mut buffer = ScanBuf::new();
        scanner.move_to_buffer(&mut buffer);

        while let Some(c) = scanner.peek() {
            if c.is_ascii_alphanumeric() {
                scanner.move_to_buffer(&mut buffer);
            } else {
                break;
            }
        }

        let literal: String = buffer.to_str();
        let kind = match LexemeMaker::match_keyword(&literal) {
            Some(keyword_kind) => keyword_kind,
            _ => Identifier
        };

        ScanResult::Token(
            buffer.to_token(kind)
        )
    }

    pub fn make_operator(scanner: &mut Scanner) -> ScanResult {
        let mut buffer = ScanBuf::new();
        scanner.move_to_buffer(&mut buffer);
        let mut operator = buffer.to_str();


        let next = scanner.peek().unwrap();
        if next == '=' || next == '/' {
            operator = format!("{}{}", operator, next.to_string());
        }

        if operator == "//" {
            scanner.next_until('\n');
            return ScanResult::Skip;
        }

        let token = match operator.as_str() {
            "=" => TokenKind::Equal,
            "==" => TokenKind::DblEqual,
            ">" => TokenKind::Greater,
            ">=" => TokenKind::GreaterEqual,
            "<" => TokenKind::Lower,
            "<=" => TokenKind::LowerEqual,
            "!" => TokenKind::Negation,
            "!=" => TokenKind::NotEqual,
            "*" => TokenKind::Asterisk,
            "/" => TokenKind::Slash,
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "(" => TokenKind::ParOpen,
            ")" => TokenKind::ParClose,
            "{" => TokenKind::BlockOpen,
            "}" => TokenKind::BlockClose,
            _ => {
                // Stops and returns error
                return ScanResult::Error(format!("Operator {} is not accepted", operator));
            }
        };

        ScanResult::Token(PartialToken {
            kind: token,
            lexeme: operator,
        })
    }

    pub fn match_keyword(literal: &str) -> Option<TokenKind> {
        match literal {
            "if" => Some(TokenKind::If),
            "elif" => Some(TokenKind::Elif),
            "else" => Some(TokenKind::Else),
            "return" => Some(TokenKind::Return),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "let" => Some(TokenKind::Let),
            _ => None,
        }
    }
}