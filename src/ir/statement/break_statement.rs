use crate::ir::IrNode;
use crate::ir::marker::Statement;
use crate::prelude::{Interpreter, Value};
use crate::runtime::Exception;

// TODO: Support labeled breaks
#[derive(Clone, Debug)]
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


impl Statement for BreakStatement {}
