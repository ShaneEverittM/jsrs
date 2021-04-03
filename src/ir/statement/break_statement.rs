use crate::{
    ir::{marker::Statement, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

// TODO: Support labeled breaks
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct BreakStatement;

impl BreakStatement {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl IrNode for BreakStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}BreakStatement\n", indent_str)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.notify_break();
        Ok(Value::Undefined)
    }
}
