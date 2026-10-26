use std::path::Path;

pub fn compile(source_file: &Path) {
    if std::fs::metadata(source_file).is_err() {
        eprintln!("[ERROR] File '{}' doesn't exist", source_file.display());
        return;
    }

    let content = std::fs::read_to_string(source_file).expect("[CRITICAL] Failed to read file");

    // TODO: lexer -> parser -> ast -> codegen

    println!("Source loaded: {} characters", content.len());
}
