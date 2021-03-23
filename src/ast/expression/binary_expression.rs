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
        use Value::*;
        let lhs_val = self.lhs.evaluate(interpreter);
        let rhs_val = self.rhs.evaluate(interpreter);

        match (lhs_val, rhs_val) {
            (Number(lhs_num), Number(rhs_num)) => match self.op {
                BinaryOp::Add => Value::Number(lhs_num + rhs_num),
                BinaryOp::Subtract => Value::Number(lhs_num - rhs_num),
            },
            // TODO: Some sort of crash mechanism
            (Undefined, Number(val)) => panic!("Attempt to add Undefined with {}", val),
            _ => panic!("Unsupported binary operation"),
        }
    }
}
impl Expression for BinaryExpression {}
