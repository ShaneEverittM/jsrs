use crate::{
    ir::{marker::Expression, IRNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct Literal {
    pub val: Value,
}

impl Literal {
    pub fn new(val: Value) -> Self {
        Self { val }
    }

    pub fn boxed(val: Value) -> Box<Self> {
        Box::new(Self { val })
    }
}
impl IRNode for Literal {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let output = format!("{}{}\n", indent_str, self.val);
        output
    }

    fn evaluate(&mut self, _interpreter: &mut Interpreter) -> Value {
        self.val.clone()
    }
}


impl Expression for Literal {}
