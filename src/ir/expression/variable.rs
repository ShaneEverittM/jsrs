use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{Exception, Interpreter, Value},
};

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

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.get_variable(&self.name)
    }

    fn edit_lvalue(
        &mut self,
        interpreter: &mut Interpreter,
        edit: Box<dyn FnOnce(&mut Value) -> Result<Value, Exception>>,
    ) -> Result<Value, Exception> {
        interpreter.edit_variable(&self.name, edit)
    }
}

impl Expression for Variable {}
