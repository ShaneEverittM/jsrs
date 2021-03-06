use crate::runtime::{Exception, Interpreter, Value};

pub mod expression;
pub mod marker;
pub mod ops;
pub mod statement;

pub trait IrNode {
    fn dump(&self, indent: u32) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception>;

    #[allow(unused_variables)]
    fn assign(&mut self, interpreter: &mut Interpreter, value: Value) -> Result<Value, Exception> {
        Err(Exception::TypeError("Not an lvalue".to_owned()))
    }
}
