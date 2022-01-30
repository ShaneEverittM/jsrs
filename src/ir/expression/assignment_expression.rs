use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct AssignmentExpression {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl AssignmentExpression {
    pub fn boxed(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self { lhs, rhs })
    }
}

impl IrNode for AssignmentExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}AssignmentExpression: \n", indent_str);
        output += &self.lhs.dump(indent + 1);
        output += &self.rhs.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let value = self.rhs.evaluate(interpreter)?;

        self.lhs.assign(interpreter, value)
    }
}
