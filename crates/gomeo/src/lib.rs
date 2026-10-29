mod ast;
mod codegen;
mod compiler;
mod lexer;
mod parser;
mod semantic;
mod types;

pub use compiler::compile;
