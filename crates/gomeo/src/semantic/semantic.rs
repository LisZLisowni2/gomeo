use super::symbols;
use crate::{
    ast::structs::{
        Expression, FunctionalDecl, Program,
        Statement::{self},
    },
    semantic::symbols::FunctionSymbol,
};

pub struct SemanticAnalyzer {
    program: Program,
    symbol_table: symbols::SymbolTable,
    current: usize,
}

impl SemanticAnalyzer {
    pub fn new(program: Program, symbol_table: symbols::SymbolTable) -> Self {
        Self {
            program,
            symbol_table,
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

        if self.search_in_symbols(&function.name).is_none() {
            return Err(format!(
                "Function with name '{}' doesn't exist in provided namespaces.",
                function.name
            ));
        }

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
            Statement::Expression(expression) => match expression {
                Expression::Call { callee, arguments } => {
                    if let Some(function_symbol) = self.search_in_symbols(callee) {
                    } else {
                        return Err(format!("Unknown function '{}'", callee));
                    };
                }
                _ => return Err(format!("Unknown expression.")),
            },
            _ => return Err(format!("Unknown statement.")),
        }
        Ok(())
    }

    fn search_in_symbols(&self, name: &str) -> Option<&FunctionSymbol> {
        self.symbol_table.search(name)
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
