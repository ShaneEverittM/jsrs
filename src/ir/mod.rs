pub mod expression;
pub mod marker;
pub mod ops;
pub mod statement;

use crate::runtime::{Interpreter, Value};

pub trait IRNode {
    fn dump(&self, indent: u32) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value;
}
