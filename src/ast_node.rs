use crate::value::Value;
use crate::interpreter::Interpreter;

pub trait ASTNode {
    fn dump(&self) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value;
}