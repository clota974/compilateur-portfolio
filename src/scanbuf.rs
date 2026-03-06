use crate::token_types::{PartialToken, TokenKind};

pub struct ScanBuf {
    pub buffer: Vec<char>
}

impl ScanBuf {
    pub fn new() -> ScanBuf {
        ScanBuf { buffer: Vec::new() }
    }

    pub fn push(&mut self, c: char) {
        self.buffer.push(c);
    }

    pub fn to_str(&self) -> String {
        self.buffer.iter().collect()
    }

    pub fn to_token(&self, kind: TokenKind) -> PartialToken {
        let lexeme = self.to_str();
        PartialToken {
            kind,
            lexeme
        }
    }
}