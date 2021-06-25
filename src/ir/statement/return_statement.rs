use crate::{
    ir::{
        IrNode,
        marker::{Expression, Statement},
    },
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct ReturnStatement {
    expression: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn boxed(expr: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self {
            expression: Some(expr),
        })
    }

    pub fn boxed_empty() -> Box<Self> {
        Box::new(Self { expression: None })
    }
}

impl IrNode for ReturnStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}ReturnStatement\n", indent_str);
        output += &self.expression.as_ref().unwrap().dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        if let Some(mut expr) = self.expression.take() {
            if let Ok(val) = expr.evaluate(interpreter) {
                interpreter.set_return_val(val);
            }
            interpreter.notify_return();
        }
        success!()
    }
}
