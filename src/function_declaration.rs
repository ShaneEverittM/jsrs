use crate::ast_node::ASTNode;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::marker::{Declaration, Statement};
use crate::scope_node::Block;
use crate::value::Value;

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
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let function = Function::new(self.name.clone(), self.body.clone());
        interpreter.global_object.put(self.name.clone(), Value::Object(function));
        Value::Undefined
    }
}
impl Statement for FunctionDeclaration {}
impl Declaration for FunctionDeclaration {}
