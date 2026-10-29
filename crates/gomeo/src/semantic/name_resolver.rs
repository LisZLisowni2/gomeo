use crate::semantic::symbols::{FunctionSymbol, SymbolTable};

pub struct NameResolver {
    symbol_table: SymbolTable,
}

impl NameResolver {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn resolve(&self, name: &str) -> Result<&FunctionSymbol, String> {
        if let Some(symbol) = self.search(name) {
            return Ok(symbol);
        }

        if let Some(symbol) = self.search(&format!("core::{}", name)) {
            return Ok(symbol);
        }

        Err(format!("Function with name '{}' doesn't exist", name))
    }

    pub fn search(&self, name: &str) -> Option<&FunctionSymbol> {
        self.symbol_table.search(name)
    }
}
