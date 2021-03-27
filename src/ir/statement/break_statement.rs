use crate::ir::IRNode;
use crate::ir::marker::Statement;
use crate::prelude::{Interpreter, Value};

// TODO: Support labeled breaks
#[derive(Clone, Debug)]
pub struct BreakStatement;

impl BreakStatement {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}


impl IRNode for BreakStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}BreakStatement\n", indent_str)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        interpreter.notify_break();
        None
    }
}


impl Statement for BreakStatement {}
