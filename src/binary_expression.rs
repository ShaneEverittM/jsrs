use crate::ast_node::ASTNode;
use crate::binary_op::BinaryOp;
use crate::interpreter::Interpreter;
use crate::marker::Expression;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    op: BinaryOp,
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn new(op: BinaryOp, lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self { op, lhs, rhs })
    }
}
impl ASTNode for BinaryExpression {
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let lhs_val = self.lhs.evaluate(interpreter);
        let rhs_val = self.rhs.evaluate(interpreter);
        if let Value::Number(lhs_num) = lhs_val {
            if let Value::Number(rhs_num) = rhs_val {
                Value::Number(lhs_num + rhs_num)
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    }
}
impl Expression for BinaryExpression {}
