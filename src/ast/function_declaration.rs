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
    fn dump(&self) -> String {
        unimplemented!()
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