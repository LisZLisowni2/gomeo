use std::path::Path;

use inkwell::context::Context;

use crate::{
    codegen::{generator::Generator, linker::compile_to_executable},
    semantic::collector::SymbolCollector,
};

pub fn compile(source_file: &Path) -> Result<(), String> {
    if std::fs::metadata(source_file).is_err() {
        return Err(format!("File '{}' doesn't exist", source_file.display()));
    }

    let content = std::fs::read_to_string(source_file).expect("[CRITICAL] Failed to open a file");

    let mut lexer = super::lexer::Lexer::new(content.as_str());
    let tokens = lexer.tokenize()?;

    let mut parser = super::parser::Parser::new(tokens);
    let program = parser.parse()?;

    let mut collector = SymbolCollector::new(&program);
    collector.collect()?;
    let symbol_table = collector.into_symbol_table();

    let mut semantic_analyzer = super::semantic::SemanticAnalyzer::new(&program, &symbol_table)?;
    semantic_analyzer.analyze()?;

    let save_ll_file_path = Path::new("/tmp/hello.ll");
    let output_file_path = Path::new("a.out");

    let context = Context::create();
    let mut code_generator = Generator::new(&context);
    code_generator.generate(&program, &symbol_table, save_ll_file_path)?;
    // compile_to_executable(save_ll_file_path, output_file_path)?;

    Ok(())
}
