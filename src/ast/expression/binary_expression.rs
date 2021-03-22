use crate::{
    ast::{marker::Expression, ops::BinaryOp, ASTNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    op: BinaryOp,
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn boxed(op: BinaryOp, lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self { op, lhs, rhs })
    }
}
impl ASTNode for BinaryExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.op);
        output += &self.lhs.dump(indent + 1);
        output += &self.rhs.dump(indent + 1);
        output
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
