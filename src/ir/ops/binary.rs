use std::fmt;

use resast::BinaryOp;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Plus,
    Minus,
    Times,
    Over,
    Or,
    And,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => f.write_str("+"),
            BinaryOperator::Minus => f.write_str("-"),
            BinaryOperator::Equal => f.write_str("=="),
            BinaryOperator::NotEqual => f.write_str("!="),
            BinaryOperator::StrictEqual => f.write_str("==="),
            BinaryOperator::StrictNotEqual => f.write_str("!==="),
            BinaryOperator::LessThan => f.write_str("<"),
            BinaryOperator::GreaterThan => f.write_str(">"),
            BinaryOperator::LessThanEqual => f.write_str("<="),
            BinaryOperator::GreaterThanEqual => f.write_str(">="),
            BinaryOperator::Times => f.write_str("*"),
            BinaryOperator::Over => f.write_str("/"),
            BinaryOperator::Or => f.write_str("||"),
            BinaryOperator::And => f.write_str("&&"),
        }
    }
}

impl From<resast::BinaryOp> for BinaryOperator {
    fn from(bin_op: BinaryOp) -> Self {
        match bin_op {
            BinaryOp::Equal => Self::Equal,
            BinaryOp::NotEqual => Self::NotEqual,
            BinaryOp::StrictEqual => Self::StrictEqual,
            BinaryOp::StrictNotEqual => Self::StrictNotEqual,
            BinaryOp::LessThan => Self::LessThan,
            BinaryOp::GreaterThan => Self::GreaterThan,
            BinaryOp::LessThanEqual => Self::LessThanEqual,
            BinaryOp::GreaterThanEqual => Self::GreaterThanEqual,
            BinaryOp::Plus => Self::Plus,
            BinaryOp::Minus => Self::Minus,
            BinaryOp::Times => Self::Times,
            BinaryOp::Over => Self::Over,
            BinaryOp::Or => Self::Or,
            BinaryOp::And => Self::And,
            _ => unimplemented!(),
        }
    }
}
