use std::collections::HashMap;
use std::path::Path;

use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicTypeEnum::{IntType, PointerType};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

use crate::ast::structs::{Expression, FunctionBody, Program, Statement};
use crate::semantic::symbols::{FunctionOrigin, SymbolTable};
use crate::types::Type;

#[derive(PartialEq, Eq)]
enum ExpressionResult<'a> {
    None,
    GlobalValue(inkwell::values::GlobalValue<'a>),
    IntValue(inkwell::values::IntValue<'a>),
    Variable(inkwell::values::BasicValueEnum<'a>),
}

pub struct Generator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, BasicValueEnum<'ctx>>,
}

impl<'ctx> Generator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("gomeo");
        let builder = context.create_builder();
        let variables = HashMap::new();

        Self {
            context,
            module,
            builder,
            variables,
        }
    }

    fn map_argument_type(&self, ty: &Type) -> Result<BasicMetadataTypeEnum<'ctx>, String> {
        match ty {
            Type::Integer32 => Ok(self.context.i32_type().into()),
            Type::String => Ok(self.context.ptr_type(AddressSpace::default()).into()),
            Type::Void => Err("Cannot use void as a basic type.".to_string()),
        }
    }

    fn map_basic_type(&self, ty: &Type) -> Result<BasicTypeEnum<'ctx>, String> {
        match ty {
            Type::Integer32 => Ok(self.context.i32_type().into()),
            Type::String => Ok(self.context.ptr_type(AddressSpace::default()).into()),
            Type::Void => Err("Cannot use void as a basic type.".to_string()),
        }
    }

    fn declare_functions(&mut self, symbol_table: &SymbolTable) -> Result<(), String> {
        for (symbol_name, symbol) in symbol_table.all_symbols() {
            let mut arguments_mapped: Vec<BasicMetadataTypeEnum<'ctx>> = vec![];
            for argument in &symbol.arguments {
                let basic_type = self.map_argument_type(&argument.arg_type)?;
                arguments_mapped.push(basic_type);
            }

            let fn_type = match &symbol.return_type {
                Type::Void => self.context.void_type().fn_type(&arguments_mapped, false),
                _ => self
                    .map_basic_type(&symbol.return_type)?
                    .fn_type(&arguments_mapped, false),
            };

            let linkage: Option<Linkage> = if symbol.origin == FunctionOrigin::External {
                Some(Linkage::External)
            } else {
                None
            };

            let final_name = if symbol_name == "main" {
                "main_gomeo"
            } else {
                symbol_name
            };
            self.module.add_function(final_name, fn_type, linkage);
        }

        Ok(())
    }

    fn generate_wrapper(&mut self) -> Result<(), String> {
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);

        let main_gomeo_fn = self
            .module
            .get_function("main_gomeo")
            .ok_or("Function main_gomeo wasn't generated".to_string())?;
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);

        if let Err(e) = self
            .builder
            .build_call(main_gomeo_fn, &[], "main_gomeo_call")
        {
            return Err(format!("Failed to generate main wrapper: {}", e));
        }

        let ret = i32_type.const_int(0, false);
        if let Err(e) = self.builder.build_return(Some(&ret)) {
            return Err(format!("Failed to generate return: {}", e));
        }

        Ok(())
    }

    fn prepare_runtime(&mut self, symbol_table: &SymbolTable) -> Result<(), String> {
        for (symbol_name, symbol) in symbol_table.all_symbols() {
            if symbol_name == "println" && symbol.origin == FunctionOrigin::External {
                let i32_type = self.context.i32_type();
                let i8_ptr_type = self.context.ptr_type(AddressSpace::default());

                let puts = i32_type.fn_type(&[i8_ptr_type.into()], false);
                self.module
                    .add_function("puts", puts, Some(inkwell::module::Linkage::External));
            }
        }

        Ok(())
    }

    fn generate_function_bodies(
        &mut self,
        program: &Program,
        symbol_table: &SymbolTable,
    ) -> Result<(), String> {
        for function in &program.functions {
            self.variables.clear();
            let function_generated = if &function.name == "main" {
                self.module
                    .get_function("main_gomeo")
                    .ok_or("Failed to find 'main_gomeo' function".to_string())?
            } else {
                self.module
                    .get_function(&function.name)
                    .ok_or(format!("Failed to find '{}' function", function.name))?
            };

            match &function.body {
                FunctionBody::Defined(block) => {
                    for (i, argument) in function.arguments.iter().enumerate() {
                        self.variables.insert(
                            argument.name.clone(),
                            function_generated
                                .get_nth_param(i as u32)
                                .ok_or(format!("Failed to retrieve {} parametr", i))?,
                        );
                    }

                    let entry = self.context.append_basic_block(function_generated, "entry");
                    self.builder.position_at_end(entry);
                    for statement in &block.statements {
                        self.generate_statement(statement, symbol_table)?
                    }

                    match function.return_type {
                        Type::Void => {
                            if let Err(e) = self.builder.build_return(None) {
                                return Err(format!(
                                    "Failed to build return for function '{}': {}",
                                    function.name, e
                                ));
                            }
                        }
                        _ => {}
                    };
                }
                FunctionBody::External => continue,
            }
        }

        Ok(())
    }

    fn generate_statement(
        &mut self,
        statement: &Statement,
        symbol_table: &SymbolTable,
    ) -> Result<(), String> {
        match statement {
            Statement::Expression(expr) => self.generate_expression(expr, symbol_table)?,
        };
        Ok(())
    }

    fn generate_expression(
        &mut self,
        expr: &Expression,
        symbol_table: &SymbolTable,
    ) -> Result<ExpressionResult<'ctx>, String> {
        match expr {
            Expression::Call { callee, arguments } => {
                let mut args_map_result: Vec<ExpressionResult> = vec![];
                for arg in arguments {
                    let mapped = self.generate_expression(arg, symbol_table)?;
                    args_map_result.push(mapped);
                }

                let mut args: Vec<BasicMetadataValueEnum> = vec![];

                for arg in args_map_result {
                    let mapped_arg = match arg {
                        ExpressionResult::GlobalValue(glob) => {
                            BasicMetadataValueEnum::from(glob.as_pointer_value())
                        }
                        ExpressionResult::IntValue(int) => BasicMetadataValueEnum::from(int),
                        ExpressionResult::Variable(var) => match var.get_type() {
                            IntType(ty) => BasicMetadataValueEnum::from(var),
                            PointerType(ty) => BasicMetadataValueEnum::from(var),
                            _ => {
                                return Err(format!(
                                    "Type '{}' is not implemented yet",
                                    var.get_type()
                                ));
                            }
                        },
                        ExpressionResult::None => {
                            return Err("Cannot use None as an argument".to_string());
                        }
                    };
                    args.push(mapped_arg);
                }

                if let Some(symbol) = symbol_table.search(callee) {
                    if args.len() != symbol.arguments.len() {
                        return Err(format!("Incorrect number of arguments for '{}'", callee));
                    }

                    // TODO: Checking types of arguments (maybe in semantic analysis)

                    match symbol.origin {
                        FunctionOrigin::External => {
                            if symbol.name == "println" {
                                let puts_fun = self
                                    .module
                                    .get_function("puts")
                                    .ok_or("Failed to find 'puts' function")?;

                                if let Err(e) = self.builder.build_call(puts_fun, &args, callee) {
                                    return Err(format!(
                                        "Failed to generate '{}' function: {}",
                                        callee, e
                                    ));
                                }
                            }
                        }
                        FunctionOrigin::UserDefined => {
                            let function = self
                                .module
                                .get_function(&symbol.name)
                                .ok_or(format!("Failed to find '{}' function", symbol.name))?;

                            if let Err(e) = self.builder.build_call(function, &args, &symbol.name) {
                                return Err(format!("Failed to generate '{}': {}", symbol.name, e));
                            }
                        }
                        FunctionOrigin::Builtin => {
                            return Err("Builtin functions not implemented yet".to_string());
                        }
                    }
                } else {
                    return Err(format!("Function with name '{}' not found", callee));
                }
            }
            Expression::StringLiteral(text) => {
                let text_build = self.builder.build_global_string_ptr(text, "text");
                if let Err(e) = text_build {
                    return Err(format!("Failed to generate string literal: {}", e));
                } else if let Ok(txt) = text_build {
                    return Ok(ExpressionResult::GlobalValue(txt));
                }
            }
            Expression::Integer32Literal(num) => {
                let i32_type = self.context.i32_type();
                let i32_module = i32_type.const_int(*num as u64, true);
                return Ok(ExpressionResult::IntValue(i32_module));
            }
            Expression::Variable(var_name) => {
                let var_value = self
                    .variables
                    .get(var_name)
                    .ok_or(format!("Failed to find variable '{}'", var_name))?;

                return Ok(ExpressionResult::Variable(*var_value));
            }
        }

        Ok(ExpressionResult::None)
    }

    pub fn generate(
        &mut self,
        program: &Program,
        symbol_table: &SymbolTable,
        save_path: &Path,
    ) -> Result<(), String> {
        self.declare_functions(symbol_table)?;
        self.prepare_runtime(symbol_table)?;
        self.generate_function_bodies(program, symbol_table)?;
        self.generate_wrapper()?;

        println!("{}", self.module.print_to_string().to_string());
        if let Err(e) = self.module.print_to_file(save_path) {
            return Err(format!("Failed to save .ll file: {}", e));
        }
        Ok(())
    }
}
