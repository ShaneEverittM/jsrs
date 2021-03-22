use crate::ast::marker::Expression;
use crate::ast::ASTNode;
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

impl ASTNode for Variable {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let output = format!("{}{}\n", indent_str, self.name);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let current_scope_vars = interpreter
            .scope_stack
            .last_mut()
            .cloned()
            .unwrap()
            .variables;
        let me = current_scope_vars
            .iter()
            .find(|v| v.name == self.name)
            .expect("Variable not in scope");
        me.value.val.clone()
    }
}
impl Expression for Variable {}
