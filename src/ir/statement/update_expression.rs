use crate::{
    ir::{expression::Variable, IrNode, marker::Expression, ops::UnaryOperator},
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
            let mut return_value = variable.clone();
            match self.op {
                UnaryOperator::Increment => match variable {
                    Value::Number(n) => *n += 1f64,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
            if self.prefix {
                return_value = variable.clone()
            }
            Ok(return_value)
        })
    }
}

impl Expression for UpdateExpression {}
