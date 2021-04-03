use crate::runtime::{Exception, Interpreter, Value};

pub mod expression;
pub mod marker;
pub mod ops;
pub mod statement;

pub trait IrNode {
    fn dump(&self, indent: u32) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception>;
    fn edit_lvalue(
        &mut self,
        _interpreter: &mut Interpreter,
        _edit: Box<dyn FnOnce(&mut Value) -> Result<Value, Exception>>,
    ) -> Result<Value, Exception> {
        unimplemented!("Not a valid lvalue")
    }
}
