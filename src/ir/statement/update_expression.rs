use crate::ir::expression::Variable;
use crate::ir::IrNode;
use crate::ir::marker::Expression;
use crate::ir::ops::UnaryOperator;
use crate::prelude::{Interpreter, Value};
use crate::runtime::Exception;

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
        let current_value = interpreter
            .resolve_variable(&self.variable.name)
            .expect("Cannot resolve variable");

        let mut return_value = current_value.clone();
        match self.op {
            UnaryOperator::Increment => match current_value {
                Value::Number(n) => *n += 1f64,
                _ => unimplemented!(),
            },
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
        if self.prefix {
            return_value = current_value.clone()
        }
        Ok(return_value)
    }
}

impl Expression for UpdateExpression {}
