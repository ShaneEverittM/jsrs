use crate::ir::marker::{Expression, Statement};
use crate::ir::IRNode;
use crate::runtime::{Interpreter, Value};

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: String,
    // TODO: Can be an expression
    pub value: Box<dyn Expression>,
}

impl VariableDeclaration {
    pub fn new(name: &str, value: Box<dyn Expression>) -> Self {
        Self {
            name: name.to_owned(),
            value,
        }
    }

    pub fn boxed(name: &str, value: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            value,
        })
    }
}

impl IRNode for VariableDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}VariableDeclaration: {}\n", indent_str, self.name);
        output += &self.value.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let value = self.value.evaluate(interpreter);
        let current_scope = interpreter.scope_stack.last_mut().unwrap();
        current_scope.insert(self.name.clone(), value);
        Value::Undefined
    }
}

impl Statement for VariableDeclaration {}
