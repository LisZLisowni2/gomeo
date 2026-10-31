use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use crate::ast::structs::Program;
use crate::semantic::symbols::SymbolTable;

pub struct Generator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Generator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("gomeo");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
        }
    }

    fn prepare_runtime(&mut self, symbol_table: &SymbolTable) -> Result<(), String> {
        Ok(())
    }

    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        Ok(())
    }
}
