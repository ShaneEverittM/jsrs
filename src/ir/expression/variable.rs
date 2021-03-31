use crate::ir::IrNode;
use crate::ir::marker::Expression;
use crate::runtime::{Interpreter, Value};

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    pub fn boxed(name: &str) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
        })
    }
}

impl IrNode for Variable {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let output = format!("{}{}\n", indent_str, self.name);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        interpreter.resolve_variable(&self.name).map(|v| v.clone())
    }
}

impl Expression for Variable {}
