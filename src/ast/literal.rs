use super::{ASTNode, Expression, Value};
use crate::runtime::Interpreter;

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
