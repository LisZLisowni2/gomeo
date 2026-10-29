use std::path::Path;

use crate::{
    ast::structs::{Expression, Statement},
    semantic::symbols::{FunctionArgument, FunctionSymbol},
};

fn prepare_symbols(
    program: &super::ast::structs::Program,
) -> Result<super::semantic::symbols::SymbolTable, String> {
    let mut symbol_table = super::semantic::symbols::create_core_symbols()?;

    for function in &program.functions {
        let mut function_arguments = vec![];

        for statement in &function.body.statements {
            match statement {
                Statement::Expression(expression) => match expression {
                    Expression::Call {
                        callee: _callee,
                        arguments,
                    } => {
                        for argument in arguments {
                            match argument {
                                Expression::StringLiteral(name) => {
                                    function_arguments.push(FunctionArgument {
                                        name: name.to_string(),
                                        arg_type: crate::types::Type::String,
                                    })
                                }
                                Expression::Integer32Literal(name) => {
                                    function_arguments.push(FunctionArgument {
                                        name: name.to_string(),
                                        arg_type: crate::types::Type::Integer32,
                                    })
                                }
                                _ => return Err("Unknown argument.".to_string()),
                            }
                        }
                    }
                    _ => return Err("Unknown expression".to_string()),
                },
                _ => return Err("Unknown statement.".to_string()),
            }
        }

        symbol_table.insert(
            function.name.clone(),
            FunctionSymbol {
                name: function.name.clone(),
                return_type: function.return_type.clone(),
                arguments: function_arguments,
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

    // TODO: lexer -> parser -> ast -> codegen
    let mut lexer = super::lexer::Lexer::new(content.as_str());
    let tokens = lexer.tokenize()?;

    let mut parser = super::parser::Parser::new(tokens);
    let program = parser.parse()?;
    let symbol_table = prepare_symbols(&program)?;

    let mut semantic_analyzer = super::semantic::SemanticAnalyzer::new(program, symbol_table);
    semantic_analyzer.analyze()?;

    println!("Source loaded: {} characters", content.len());

    Ok(())
}
