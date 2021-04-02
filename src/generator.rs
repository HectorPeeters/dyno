use crate::ast::{AstNode, BinaryOperationType};
use crate::error::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub struct CodeGenerator<'a> {
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    execution_engine: ExecutionEngine<'a>,
}

impl CodeGenerator<'_> {
    pub fn jit_execute(&self, ast: &AstNode) -> DynoResult<u64> {
        Ok(0)
    }
}

pub fn compile_and_run(ast: &AstNode) -> DynoResult<u64> {
    let context = Context::create();
    let module = context.create_module("jit");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let code_generator = CodeGenerator {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    code_generator.jit_execute(ast)
}
