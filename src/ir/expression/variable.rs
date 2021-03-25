use crate::ir::marker::Expression;
use crate::ir::IRNode;
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

impl IRNode for Variable {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let output = format!("{}{}\n", indent_str, self.name);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let mut scope_stack = interpreter.scope_stack.clone();
        for scope in scope_stack.iter_mut().rev() {
            match scope.get(&self.name) {
                None => continue,
                Some(val) => return val.clone(),
            }
        }
        Value::Undefined
    }
}
impl Expression for Variable {}
