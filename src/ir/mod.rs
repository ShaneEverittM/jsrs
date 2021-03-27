use crate::runtime::{Interpreter, Value};

pub mod expression;
pub mod marker;
pub mod ops;
pub mod statement;

pub trait IrNode {
    fn dump(&self, indent: u32) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value>;
}
