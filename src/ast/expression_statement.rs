use super::{ASTNode, Expression, Statement, Value};
use crate::runtime::Interpreter;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    expr: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(expr: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self { expr })
    }
}

impl ASTNode for ExpressionStatement {
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        self.expr.evaluate(interpreter)
    }
}
impl Statement for ExpressionStatement {}
