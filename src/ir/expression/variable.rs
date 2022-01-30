use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{Exception, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
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

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.variable(&self.name)
    }

    fn assign(&mut self, interpreter: &mut Interpreter, value: Value) -> Result<Value, Exception> {
        interpreter.assign_variable(&self.name, value)
    }
}
