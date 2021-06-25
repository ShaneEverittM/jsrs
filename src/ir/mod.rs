use crate::runtime::{Exception, Interpreter, Value};

pub mod expression;
pub mod marker;
pub mod ops;
pub mod statement;

pub trait IrNode {
    fn dump(&self, indent: u32) -> String;
    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception>;

    /// When you need to assign to the result of an expression, this first evaluates the expression,
    /// then the interpreter finds the actual variable before applying the closure to it.
    #[allow(unused_variables)]
    fn edit_lvalue(
        &mut self,
        interpreter: &mut Interpreter,
        edit: Box<dyn FnOnce(&mut Value) -> Result<Value, Exception>>,
    ) -> Result<Value, Exception> {
        Err(Exception::TypeError("Not an lvalue".to_owned()))
    }
}
