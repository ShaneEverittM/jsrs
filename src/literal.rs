use crate::ast_node::ASTNode;
use crate::interpreter::Interpreter;
use crate::marker::Expression;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Literal {
    val: Value,
}

impl Literal {
    pub fn new(val: Value) -> Box<Self> {
        Box::new(Self { val })
    }
}
impl ASTNode for Literal {
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, _interpreter: &mut Interpreter) -> Value {
        self.val.clone()
    }
}
impl Expression for Literal {}
