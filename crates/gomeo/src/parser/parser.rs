use crate::ast::structs::{Block, Expression, FunctionalDecl, Program, Statement};
use crate::lexer::token::{Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();

        while !self.check(&TokenKind::Eof) {
            functions.push(self.parse_function()?);
        }

        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<FunctionalDecl, String> {
        self.consume(&TokenKind::Fun, "Expected 'fun'")?;

        let name = self
            .consume(&TokenKind::Identifier, "Expected function name")?
            .lexeme;

        self.consume(&TokenKind::LeftParen, "Expected '('")?;
        self.consume(&TokenKind::RightParen, "Expected ')'")?;

        let body = self.parse_body()?;

        Ok(FunctionalDecl { name, body })
    }

    fn parse_body(&mut self) -> Result<Block, String> {
        self.consume(&TokenKind::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::Eof) {
            statements.push(self.parse_statement()?);
        }

        self.consume(&TokenKind::RightBrace, "Expected '}'")?;

        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        let expression = self.parse_expression()?;

        self.consume(&TokenKind::Semicolon, "Expected ';'")?;

        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        if self.check(&TokenKind::StringLiteral) {
            let token = self.advance();

            let value = token
                .lexeme
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| {
                    format!(
                        "Invalid string at line {}, column {}",
                        token.line, token.column
                    )
                })?
                .to_string();

            return Ok(Expression::StringLiteral(value));
        }

        if self.check(&TokenKind::Identifier) {
            return self.parse_call();
        }

        let token = self.peek();

        Err(format!(
            "
            Unexcepted expression at line {}, column {}
        ",
            token.line, token.column
        ))
    }

    fn parse_call(&mut self) -> Result<Expression, String> {
        let callee = self
            .consume(&TokenKind::Identifier, "Expected function name")?
            .lexeme;

        self.consume(&TokenKind::LeftParen, "Expected '('")?;

        let mut arguments = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            arguments.push(self.parse_expression()?);

            while self.check(&TokenKind::Comma) {
                self.advance();
                arguments.push(self.parse_expression()?);
            }
        }

        self.consume(&TokenKind::RightParen, "Expected ')'")?;

        Ok(Expression::Call { callee, arguments })
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();

        if token.kind != TokenKind::Eof {
            self.current += 1;
        }

        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<Token, String> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let token = self.peek();

            Err(format!(
                "{}, line: {}, column: {}",
                message, token.line, token.column
            ))
        }
    }
}
