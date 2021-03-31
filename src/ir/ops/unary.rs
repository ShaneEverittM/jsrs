use std::fmt;

use resast::UnaryOp;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Increment,
    Decrement,

}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Increment => f.write_str("++"),
            UnaryOperator::Decrement => f.write_str("--"),
        }
    }
}

impl From<resast::UnaryOp> for UnaryOperator {
    fn from(bin_op: UnaryOp) -> Self {
        match bin_op {
            UnaryOp::Plus => Self::Increment,
            _ => unimplemented!(),
        }
    }
}