use super::Value;
use crate::runtime::Interpreter;

pub trait ASTNode {
    fn dump(&self) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value;
}
