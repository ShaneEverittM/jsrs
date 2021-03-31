use crate::{
    ir::{IrNode, marker::Expression, ops::BinaryOperator},
    runtime::{exception::*, Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    op: BinaryOperator,
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn boxed(
        op: BinaryOperator,
        lhs: Box<dyn Expression>,
        rhs: Box<dyn Expression>,
    ) -> Box<Self> {
        Box::new(Self { op, lhs, rhs })
    }
}

impl IrNode for BinaryExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}BinaryExpression: {}\n", indent_str, self.op);
        output += &self.lhs.dump(indent + 1);
        output += &self.rhs.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        use Value::*;
        let lhs_val = self.lhs.evaluate(interpreter).unwrap_or(Value::Undefined);
        let rhs_val = self.rhs.evaluate(interpreter).unwrap_or(Value::Undefined);

        // Should allow this here, since it's not our job as the interpreter to guess at
        // best practices for the programmer
        #[allow(clippy::float_cmp)]
            let val = match (lhs_val.clone(), rhs_val.clone()) {
            (Number(lhs_num), Number(rhs_num)) => match self.op {
                BinaryOperator::Plus => Value::Number(lhs_num + rhs_num),
                BinaryOperator::Minus => Value::Number(lhs_num - rhs_num),
                BinaryOperator::Equal => Value::Boolean(lhs_num == rhs_num),
                BinaryOperator::NotEqual => Value::Boolean(lhs_num != rhs_num),
                BinaryOperator::StrictEqual => Value::Boolean(lhs_num == rhs_num),
                BinaryOperator::StrictNotEqual => Value::Boolean(lhs_num != rhs_num),
                BinaryOperator::LessThan => Value::Boolean(lhs_num < rhs_num),
                BinaryOperator::GreaterThan => Value::Boolean(lhs_num > rhs_num),
                BinaryOperator::LessThanEqual => Value::Boolean(lhs_num <= rhs_num),
                BinaryOperator::GreaterThanEqual => Value::Boolean(lhs_num >= rhs_num),
                BinaryOperator::Times => Value::Number(lhs_num * rhs_num),
                BinaryOperator::Over => Value::Number(lhs_num / rhs_num),
                BinaryOperator::Or => {
                    panic!("Cannot or numbers")
                }
                BinaryOperator::And => {
                    panic!("Cannot and numbers")
                }
            },
            (String(lhs_str), String(rhs_str)) => match self.op {
                BinaryOperator::StrictEqual => Value::Boolean(lhs_str == rhs_str),
                _ => panic!("Unsupported string operation")
            }
            (Undefined, Undefined) => match self.op {
                BinaryOperator::Equal => Value::Boolean(true),
                BinaryOperator::StrictEqual => Value::Boolean(true),
                _ => unimplemented!("Undefined is weird")
            }
            // TODO: Some sort of crash mechanism
            (Undefined, Number(val)) => panic!("Attempt to add Undefined with {}", val),
            _ => panic!("Unsupported binary operation: {:?} {:?} {:?}", lhs_val, self.op, rhs_val),
        };
        Ok(val)
    }
}

impl Expression for BinaryExpression {}
