use crate::ast::{BinaryOperationType, Expression, Statement};
use crate::error::*;
use crate::types::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::IntValue;
use inkwell::OptimizationLevel;

type MainFunc = unsafe extern "C" fn() -> u64;

pub struct CodeGenerator<'a> {
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    execution_engine: ExecutionEngine<'a>,
}

impl CodeGenerator<'_> {
    fn generate_literal(&self, literal_type: &DynoType, value: &DynoValue) -> DynoResult<IntValue> {
        match (literal_type, value) {
            (DynoType::UInt8(), DynoValue::UInt(x)) => {
                Ok(self.context.i8_type().const_int(*x, false))
            }
            (DynoType::UInt16(), DynoValue::UInt(x)) => {
                Ok(self.context.i16_type().const_int(*x, false))
            }
            (DynoType::UInt32(), DynoValue::UInt(x)) => {
                Ok(self.context.i32_type().const_int(*x, false))
            }
            (DynoType::UInt64(), DynoValue::UInt(x)) => {
                Ok(self.context.i64_type().const_int(*x, false))
            }
            _ => Err(DynoError::GeneratorError(format!(
                "Invalid type-value pair: {:?} {:?}",
                literal_type, value
            ))),
        }
    }

    fn generate_binary_operation(
        &self,
        op_type: &BinaryOperationType,
        left: &Expression,
        right: &Expression,
    ) -> DynoResult<IntValue> {
        let left_value = self.generate_expression(left)?;
        let right_value = self.generate_expression(right)?;

        match op_type {
            BinaryOperationType::Add => Ok(self.builder.build_int_add(left_value, right_value, "")),
            BinaryOperationType::Subtract => {
                Ok(self.builder.build_int_sub(left_value, right_value, ""))
            }
            BinaryOperationType::Multiply => {
                Ok(self.builder.build_int_mul(left_value, right_value, ""))
            }
            BinaryOperationType::Divide => {
                Ok(self
                    .builder
                    .build_int_unsigned_div(left_value, right_value, ""))
            }
            _ => Err(DynoError::GeneratorError(format!(
                "Invalid binary operation: {:?}",
                op_type
            ))),
        }
    }

    fn generate_widen(
        &self,
        expression: &Expression,
        widen_type: &DynoType,
    ) -> DynoResult<IntValue> {
        let value = self.generate_expression(expression)?;

        let llvm_type = match widen_type {
            DynoType::UInt8() => Ok(self.context.i8_type()),
            DynoType::UInt16() => Ok(self.context.i16_type()),
            DynoType::UInt32() => Ok(self.context.i32_type()),
            DynoType::UInt64() => Ok(self.context.i64_type()),
            _ => Err(DynoError::GeneratorError(format!(
                "Cannot widen: {:?}",
                expression
            ))),
        };

        llvm_type.map(|x| self.builder.build_int_z_extend(value, x, ""))
    }

    fn generate_expression(&self, expression: &Expression) -> DynoResult<IntValue> {
        match expression {
            Expression::Literal(literal_type, value) => {
                self.generate_literal(&literal_type, &value)
            }
            Expression::BinaryOperation(op, left, right) => {
                self.generate_binary_operation(&op, &left, &right)
            }
            Expression::Widen(value, widen_type) => self.generate_widen(&value, &widen_type),
            _ => Err(DynoError::GeneratorError(format!(
                "Unknown expression to generate: {:?}",
                expression
            ))),
        }
    }

    fn generate_return(&self, expression: &Expression) -> DynoResult<()> {
        let expression_value = self.generate_expression(expression)?;

        let i64_type = self.context.i64_type();
        let return_value = self
            .builder
            .build_int_s_extend(expression_value, i64_type, "");

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
