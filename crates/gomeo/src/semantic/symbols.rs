use crate::types::Type;
use std::collections::HashMap;

pub struct FunctionSymbol {
    pub name: String,
    pub return_type: Type,
    pub arguments: Vec<FunctionArgument>,
}

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: Type,
}

pub fn create_core_symbols() -> Result<SymbolTable, String> {
    let mut symbol_table = SymbolTable::new();

    symbol_table.insert(
        String::from("core::println"),
        FunctionSymbol {
            name: String::from("println"),
            return_type: Type::Void,
            arguments: vec![FunctionArgument {
                name: String::from("text"),
                arg_type: Type::String,
            }],
        },
    )?;

    Ok(symbol_table)
}

pub struct SymbolTable {
    symbols: HashMap<String, FunctionSymbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol: FunctionSymbol) -> Result<(), String> {
        if self.symbols.contains_key(&name) {
            return Err(format!("The symbol with name '{}' already exists", name));
        }
        self.symbols.insert(name, symbol);
        Ok(())
    }

    pub fn search(&self, name: &str) -> Option<&FunctionSymbol> {
        self.symbols.get(name)
    }
}
