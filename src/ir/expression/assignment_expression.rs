use crate::ir::expression::Variable;
use crate::ir::IrNode;
use crate::ir::marker::Expression;
use crate::prelude::{Interpreter, Value};
use crate::runtime::exception::*;
use crate::runtime::Exception;

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    variable: Variable,
    new_value: Box<dyn Expression>,
}

impl AssignmentExpression {
    pub fn boxed(variable: Variable, new_value: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self {
            variable,
            new_value,
        })
    }
}

impl IrNode for AssignmentExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!(
            "{}AssignmentExpression: {}\n",
            indent_str, self.variable.name
        );
        output += &self.new_value.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let new_val = self.new_value.evaluate(interpreter);

        interpreter.edit_variable(&self.variable.name, |variable| {
            *variable = new_val?;
            success!()
        })
    }
}

impl Expression for AssignmentExpression {}
