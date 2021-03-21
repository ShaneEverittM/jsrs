use super::{ASTNode, Block, Declaration, Statement, Value};
use crate::runtime::{Function, Interpreter};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    body: Box<Block>,
}

impl FunctionDeclaration {
    pub fn new(name: String, body: Box<Block>) -> Box<Self> {
        Box::new(Self { name, body })
    }
}

impl ASTNode for FunctionDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}FunctionDeclaration: {}\n", indent_str, self.name);
        output += &self.body.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let function = Function::new(self.name.clone(), self.body.clone());
        interpreter
            .global_object
            .put(self.name.clone(), Value::Object(function));
        Value::Undefined
    }
}
impl Statement for FunctionDeclaration {}
impl Declaration for FunctionDeclaration {}
