use super::name_resolver::NameResolver;
use super::symbols;
use crate::ast::structs::{
    Expression, FunctionalDecl, Program,
    Statement::{self},
};

pub struct SemanticAnalyzer {
    program: Program,
    resolver: NameResolver,
    current: usize,
}

impl SemanticAnalyzer {
    pub fn new(program: Program, symbol_table: symbols::SymbolTable) -> Self {
        let resolver = NameResolver::new(symbol_table);

        Self {
            program,
            resolver,
            current: 0,
        }
    }

    pub fn analyze(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.analyze_function()?;
            self.advance();
        }

        Ok(())
    }

    fn analyze_function(&self) -> Result<(), String> {
        let function = self.peek();

        self.resolver.resolve(&function.name)?;
        self.analyze_body()?;

        Ok(())
    }

    fn analyze_body(&self) -> Result<(), String> {
        let function = self.peek();
        let body = &function.body;

        for statement in &body.statements {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn analyze_statement(&self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Expression(expression) => self.analyze_expression(expression)?,
        }
        Ok(())
    }

    fn analyze_expression(&self, expression: &Expression) -> Result<(), String> {
        match expression {
            Expression::Call { callee, arguments } => {
                let original_function = self.resolver.resolve(callee)?;

                if original_function.arguments.len() != arguments.len() {
                    return Err(format!(
                        "Expected {} arguments, received {}",
                        original_function.arguments.len(),
                        arguments.len()
                    ));
                }
            }
            _ => {
                return Err("Unknown expression".to_string());
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.program.functions.len()
    }

    fn peek(&self) -> &FunctionalDecl {
        &self.program.functions[self.current]
    }

    fn advance(&mut self) {
        self.current += 1
    }
}
