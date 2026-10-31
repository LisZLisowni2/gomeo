use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, VoidType};

use crate::ast::structs::Program;
use crate::semantic::symbols::SymbolTable;
use crate::types::Type;

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

    fn map_basic_type(&self, ty: &Type) -> Result<BasicTypeEnum<'ctx>, String> {
        match ty {
            Type::Integer32 => Ok(self.context.i32_type().into()),
            Type::String => Ok(self.context.ptr_type(AddressSpace::default()).into()),
            Type::Void => Err("Cannot use void as a basic type.".to_string()),
        }
    }

    fn map_return_type(&self, ty: &Type) -> AnyTypeEnum<'ctx> {
        match ty {
            Type::Integer32 => self.context.i32_type().into(),
            Type::String => self.context.ptr_type(AddressSpace::default()).into(),
            Type::Void => self.context.void_type().into(),
        }
    }

    fn prepare_runtime(&mut self, symbol_table: &SymbolTable) -> Result<(), String> {
        for (symbol_name, symbol) in symbol_table.all_symbols() {}

        Ok(())
    }

    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        Ok(())
    }
}
