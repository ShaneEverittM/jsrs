use crate::ast::expression::Literal;
use crate::ast::marker::Statement;
use crate::ast::ASTNode;
use crate::runtime::{Interpreter, Value};

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: String,
    // TODO: Can be an expression
    pub value: Literal,
}

impl VariableDeclaration {
    pub fn new(name: &str, value: Literal) -> Self {
        Self {
            name: name.to_owned(),
            value,
        }
    }

    pub fn boxed(name: &str, value: Literal) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            value,
        })
    }
}

impl ASTNode for VariableDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}VariableDeclaration: {}\n", indent_str, self.name);
        output += &self.value.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let current_scope = interpreter.scope_stack.last_mut().unwrap();
        current_scope.variables.push(self.clone());
        self.value.val.clone()
    }
}

impl Statement for VariableDeclaration {}
