use crate::ir::expression::Variable;
use crate::ir::marker::Expression;
use crate::ir::IRNode;
use crate::prelude::{Interpreter, Value};

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

impl IRNode for AssignmentExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!(
            "{}AssignmentExpression: {}\n",
            indent_str, self.variable.name
        );
        output += &self.new_value.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let new_val = self.new_value.evaluate(interpreter);
        let current_val = interpreter
            .resolve_variable(&self.variable.name)
            .expect("Cannot find variable");
        *current_val = new_val.clone();
        new_val
    }
}

impl Expression for AssignmentExpression {}
