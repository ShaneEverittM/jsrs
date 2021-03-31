use crate::{
    ir::{expression::Variable, marker::Expression, ops::UnaryOperator, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct UpdateExpression {
    variable: Variable,
    op: UnaryOperator,
    prefix: bool,
}

impl UpdateExpression {
    pub fn boxed(variable: Variable, op: UnaryOperator, prefix: bool) -> Box<Self> {
        Box::new(Self {
            variable,
            op,
            prefix,
        })
    }
}

impl IrNode for UpdateExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!(
            "{}UpdateExpression: {} {}\n",
            indent_str, self.variable.name, self.op
        )
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.edit_variable(&self.variable.name, |variable| {
            // If postfix operator, then cache original
            let original_value = if !self.prefix {
                variable.clone()
            } else {
                Value::Undefined
            };

            // Apply operation
            match self.op {
                UnaryOperator::Increment => match variable {
                    Value::Number(n) => *n += 1f64,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }

            // If prefix operator, return current value, otherwise the cached value
            if self.prefix {
                success!(variable.clone())
            } else {
                success!(original_value)
            }
        })
    }
}

impl Expression for UpdateExpression {}
