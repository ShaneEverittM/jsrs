use crate::{
    ir::{
        IRNode,
        marker::{Expression, Statement},
    },
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    expr: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        Self { expr }
    }
    
    pub fn boxed(expr: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self { expr })
    }
}

impl IRNode for ExpressionStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}ExpressionStatement\n", indent_str);
        output += &self.expr.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        self.expr.evaluate(interpreter)
    }
}
impl Statement for ExpressionStatement {}
