use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

#[derive(Debug, Clone)]
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
        let new_val = self.rhs.evaluate(interpreter)?;

        let edit_fn = |lvalue: &mut Value| -> Result<Value, Exception> {
            *lvalue = new_val.clone();
            Ok(new_val)
        };

        self.lhs.edit_lvalue(interpreter, Box::new(edit_fn))
    }
}

impl Expression for AssignmentExpression {}
