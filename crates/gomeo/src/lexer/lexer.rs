use super::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.start_column = self.column;

            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, String> {
        let c = self.advance();

        let kind = match c {
            ' ' | '\r' | '\t' => return Ok(None),
            '\n' => {
                self.line += 1;
                self.column = 1;
                return Ok(None);
            }
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '=' => TokenKind::Equal,
            ',' => TokenKind::Comma,

            '"' => return self.string(),

            c if c.is_ascii_digit() => {
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.advance();
                }

                TokenKind::IntegerLiteral
            }

            c if is_identifier_start(c) => {
                while self.peek().is_some_and(is_identifier_continue) {
                    self.advance();
                }

                let text = self.lexeme();

                match text.as_str() {
                    "fun" => TokenKind::Fun,
                    "mut" => TokenKind::Mut,
                    "let" => TokenKind::Let,
                    _ => TokenKind::Identifier,
                }
            }

            _ => {
                return Err(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    c, self.line, self.start_column
                ));
            }
        };

        Ok(Some(self.make_token(kind)))
    }

    fn string(&mut self) -> Result<Option<Token>, String> {
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(Some(self.make_token(TokenKind::StringLiteral)));
            }

            if c == '\n' {
                return Err(format!(
                    "Unterminated string at line {}, column {}",
                    self.line, self.start_column
                ));
            }

            self.advance();
        }

        Err(format!(
            "Unterminated string at line {}, column {}",
            self.line, self.start_column
        ))
    }

    pub fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.lexeme(),
            column: self.start_column,
            line: self.line,
        }
    }

    pub fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    pub fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
