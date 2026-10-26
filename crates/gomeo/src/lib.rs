mod ast;
mod codegen;
mod compiler;
mod lexer;
mod parser;
mod semantic;

pub use compiler::compile;
