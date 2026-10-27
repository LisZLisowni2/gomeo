use std::path::Path;

pub fn compile(source_file: &Path) -> Result<(), String> {
    if std::fs::metadata(source_file).is_err() {
        return Err(format!("File '{}' doesn't exist", source_file.display()));
    }

    let content = std::fs::read_to_string(source_file).expect("[CRITICAL] Failed to open a file");

    // TODO: lexer -> parser -> ast -> codegen
    let mut lexer = super::lexer::Lexer::new(content.as_str());
    let tokens = lexer.tokenize()?;

    for token in tokens {
        println!("{:?}", token);
    }

    println!("Source loaded: {} characters", content.len());

    Ok(())
}
