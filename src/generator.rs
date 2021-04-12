use crate::ast::{Expression, Statement};
use crate::error::*;
use crate::types::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;

type MainFunc = unsafe extern "C" fn() -> u64;

pub struct CodeGenerator<'a> {
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    execution_engine: ExecutionEngine<'a>,
}

impl CodeGenerator<'_> {
    fn generate_literal(
        &self,
        literal_type: &DynoType,
        value: &DynoValue,
    ) -> DynoResult<BasicValueEnum> {
        match (literal_type, value) {
            (DynoType::UInt8(), DynoValue::UInt(x)) => Ok(BasicValueEnum::IntValue(
                self.context.i8_type().const_int(*x, false),
            )),
            _ => Err(DynoError::GeneratorError(format!(
                "Invalid type-value pair: {:?} {:?}",
                literal_type, value
            ))),
        }
    }
    fn generate_expression(&self, expression: &Expression) -> DynoResult<BasicValueEnum> {
        match expression {
            Expression::Literal(literal_type, value) => {
                self.generate_literal(&literal_type, &value)
            }
            _ => Err(DynoError::GeneratorError(format!(
                "Unknown expression to generate: {:?}",
                expression
            ))),
        }
    }

    fn generate_return(&self, expression: &Expression) -> DynoResult<()> {
        let expression_value = self.generate_expression(expression)?;

        let i64_type = self.context.i64_type();
        let return_value =
            self.builder
                .build_int_s_extend(expression_value.into_int_value(), i64_type, "");

        self.builder.build_return(Some(&return_value));
        Ok(())
    }

    fn generate_statement(&self, statement: &Statement) -> DynoResult<()> {
        match statement {
            Statement::Return(x) => self.generate_return(x),
            _ => Err(DynoError::GeneratorError(format!(
                "Unknown statement to generate: {:?}",
                statement
            ))),
        }
    }

    pub fn jit_execute(&self, ast: &Statement) -> DynoResult<u64> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        self.generate_statement(ast)?;

        unsafe {
            let function: JitFunction<MainFunc> =
                self.execution_engine.get_function("main").unwrap();

            Ok(function.call())
        }
    }
}

pub fn compile_and_run(statement: &Statement) -> DynoResult<u64> {
    let context = Context::create();
    let module = context.create_module("jit");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let code_generator = CodeGenerator {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    code_generator.jit_execute(statement)
}
