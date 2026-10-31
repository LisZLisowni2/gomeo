use super::name_resolver::NameResolver;
use crate::{
    ast::structs::{
        Expression, FunctionBody, FunctionalDecl, Program,
        Statement::{self},
    },
    semantic::collector::SymbolCollector,
};

pub struct SemanticAnalyzer {
    program: Program,
    resolver: NameResolver,
    current: usize,
}

impl SemanticAnalyzer {
    pub fn new(program: Program) -> Result<Self, String> {
        let mut collector = SymbolCollector::new(&program);

        collector.collect()?;
        let symbol_table = collector.into_symbol_table();

        let resolver = NameResolver::new(symbol_table);

        Ok(Self {
            program,
            resolver,
            current: 0,
        })
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
        let fun_body = &function.body;

        match fun_body {
            FunctionBody::Defined(block) => {
                for statement in &block.statements {
                    self.analyze_statement(statement)?;
                }
            }
            FunctionBody::External => {}
        };

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

// TODO: Support forward function references.
#[cfg(test)]
mod tests {
    // TODO: Unittest - call function defined after main
    //
}
