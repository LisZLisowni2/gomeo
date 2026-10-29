use std::path::{Path, PathBuf};

use crate::{
    codegen::{linker::compile_to_executable, llvm::generate_code},
    semantic::symbols::FunctionSymbol,
};

fn prepare_symbols(
    program: &super::ast::structs::Program,
) -> Result<super::semantic::symbols::SymbolTable, String> {
    let mut symbol_table = super::semantic::symbols::create_core_symbols()?;

    for function in &program.functions {
        symbol_table.insert(
            function.name.clone(),
            FunctionSymbol {
                name: function.name.clone(),
                return_type: function.return_type.clone(),
                arguments: function.arguments.clone(),
            },
        )?;
    }

    Ok(symbol_table)
}

pub fn compile(source_file: &Path) -> Result<(), String> {
    if std::fs::metadata(source_file).is_err() {
        return Err(format!("File '{}' doesn't exist", source_file.display()));
    }

    let content = std::fs::read_to_string(source_file).expect("[CRITICAL] Failed to open a file");

    let mut lexer = super::lexer::Lexer::new(content.as_str());
    let tokens = lexer.tokenize()?;

    let mut parser = super::parser::Parser::new(tokens);
    let program = parser.parse()?;
    let symbol_table = prepare_symbols(&program)?;

    let mut semantic_analyzer = super::semantic::SemanticAnalyzer::new(program, symbol_table);
    semantic_analyzer.analyze()?;

    let save_ll_file_path = Path::new("/tmp/hello.ll");
    let output_file_path = Path::new("a.out");

    generate_code(save_ll_file_path)?;
    compile_to_executable(save_ll_file_path, output_file_path)?;

    Ok(())
}
