use crate::ast_node::ASTNode;
use crate::interpreter::Interpreter;
use crate::marker::Statement;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Block {
    pub children: Vec<Box<dyn Statement>>,
    // variables
    // function declarations
}

impl Default for Block {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Block {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            children: Vec::new(),
        })
    }
    pub fn append(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
    }
}
impl ASTNode for Block {
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, _interpreter: &mut Interpreter) -> Value {
        unimplemented!()
    }
}

impl Statement for Block {}
