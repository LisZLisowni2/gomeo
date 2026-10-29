#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Fun,
    Let,
    Mut,

    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    StringLiteral,

    // Special chars
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Equal,
    Comma,

    ReturnArrow,

    // End of file
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}
