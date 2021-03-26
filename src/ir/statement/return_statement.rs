use crate::{
    ir::{marker::Expression, marker::Statement, IRNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
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
        Box::new(Self {
            expression: None,
        })
    }
}

impl Default for ReturnStatement {
    fn default() -> Self {
        Self { expression: None }
    }
}

impl IRNode for ReturnStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}ReturnStatement\n", indent_str);
        output += &self.expression.as_ref().unwrap().dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        if let Some(mut expr) = self.expression.take() {
            interpreter.notify_return();
            let ret_val = expr.evaluate(interpreter);
            interpreter.set_return_val(ret_val);
        }
        Value::Undefined
    }
}

impl Statement for ReturnStatement {}
