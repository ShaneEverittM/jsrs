use crate::ir::IRNode;
use crate::ir::marker::{Expression, Statement};
use crate::runtime::{Interpreter, Value};

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Option<Box<dyn Expression>>,
}

impl VariableDeclaration {
    pub fn new(name: &str, value: Box<dyn Expression>) -> Self {
        Self {
            name: name.to_owned(),
            value: Some(value),
        }
    }

    pub fn boxed(name: &str, value: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            value: Some(value),
        })
    }
}

impl IRNode for VariableDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}VariableDeclaration: {}\n", indent_str, self.name);
        if let Some(value) = self.value.as_ref() {
            output += &value.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        let value = match self.value.as_mut() {
            None => Value::Undefined,
            Some(value) => value.evaluate(interpreter).unwrap_or(Value::Undefined),
        };
        let current_scope = interpreter.scope_stack.last_mut().unwrap();
        current_scope.insert(self.name.clone(), value);
        None
    }
}

impl Statement for VariableDeclaration {}
