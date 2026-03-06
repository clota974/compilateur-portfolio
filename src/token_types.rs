#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    StringChain,
    Number,
    Negation,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    DblEqual,
    Greater,
    GreaterEqual,
    Lower,
    LowerEqual,
    NotEqual,
    If,
    Elif,
    Else,
    Return,
    True,
    False,
    While,
    For,
    ParOpen,
    ParClose,
    BlockOpen,
    BlockClose,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct PartialToken {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl PartialToken {
    pub fn set_pos(self, line: usize, column: usize) -> Token {
        Token {
            kind: self.kind,
            lexeme: self.lexeme,
            line,
            column
        }
    }
}
