use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{Exception, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct EmptyExpression;

impl EmptyExpression {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl IrNode for EmptyExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}EmptyExpression", indent_str)
    }

    fn evaluate(&mut self, _: &mut Interpreter) -> Result<Value, Exception> {
        Ok(Value::Undefined)
    }
}
