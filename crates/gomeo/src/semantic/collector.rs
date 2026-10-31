use crate::{
    ast::structs::{FunctionBody, Program},
    semantic::symbols::{FunctionOrigin, FunctionSymbol, SymbolTable},
};

pub struct SymbolCollector<'a> {
    ast: &'a Program,
    symbol_table: SymbolTable,
}

impl<'a> SymbolCollector<'a> {
    pub fn new(ast: &'a Program) -> Self {
        Self {
            ast,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn collect(&mut self) -> Result<(), String> {
        for function in &self.ast.functions {
            let origin = match &function.body {
                FunctionBody::Defined(_body) => FunctionOrigin::UserDefined,
                FunctionBody::External => FunctionOrigin::External,
            };

            self.symbol_table.insert(
                function.name.clone(),
                FunctionSymbol {
                    name: function.name.clone(),
                    arguments: function.arguments.clone(),
                    return_type: function.return_type.clone(),
                    origin,
                },
            )?;
        }

        Ok(())
    }

    pub fn into_symbol_table(self) -> SymbolTable {
        self.symbol_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::structs::{
            Block,
            FunctionBody::{Defined, External},
            FunctionalDecl,
        },
        types::Type::Void,
    };

    #[test]
    fn one_defined_function() {
        let function = FunctionalDecl {
            name: "test".to_string(),
            arguments: vec![],
            return_type: Void,
            body: Defined(Block { statements: vec![] }),
        };
        let program = Program {
            functions: vec![function],
        };

        let mut collector = SymbolCollector::new(&program);
        assert!(collector.collect().is_ok());
        let symbol_table = collector.into_symbol_table();
        assert_eq!(symbol_table.all_symbols().len(), 1);
        assert_eq!(
            symbol_table.all_symbols().get("test").unwrap().origin,
            FunctionOrigin::UserDefined
        );
    }

    #[test]
    fn one_external_function() {
        let function = FunctionalDecl {
            name: "test".to_string(),
            arguments: vec![],
            return_type: Void,
            body: External,
        };
        let program = Program {
            functions: vec![function],
        };

        let mut collector = SymbolCollector::new(&program);
        assert!(collector.collect().is_ok());
        let symbol_table = collector.into_symbol_table();
        assert_eq!(symbol_table.all_symbols().len(), 1);
        assert_eq!(
            symbol_table.all_symbols().get("test").unwrap().origin,
            FunctionOrigin::External
        );
    }

    #[test]
    fn two_functions_with_the_same_name() {
        let function = FunctionalDecl {
            name: "test".to_string(),
            arguments: vec![],
            return_type: Void,
            body: External,
        };
        let second_function = FunctionalDecl {
            name: "test".to_string(),
            arguments: vec![],
            return_type: Void,
            body: External,
        };
        let program = Program {
            functions: vec![function, second_function],
        };

        let mut collector = SymbolCollector::new(&program);
        assert!(collector.collect().is_err());
    }

    #[test]
    fn two_functions_with_the_different_name() {
        let function = FunctionalDecl {
            name: "test".to_string(),
            arguments: vec![],
            return_type: Void,
            body: External,
        };
        let second_function = FunctionalDecl {
            name: "test2".to_string(),
            arguments: vec![],
            return_type: Void,
            body: External,
        };
        let program = Program {
            functions: vec![function, second_function],
        };

        let mut collector = SymbolCollector::new(&program);
        assert!(collector.collect().is_ok());
        assert_eq!(collector.into_symbol_table().all_symbols().len(), 2);
    }
}
