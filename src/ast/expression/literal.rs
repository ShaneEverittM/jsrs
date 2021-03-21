use crate::{
    ast::{marker::Expression, ASTNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct Literal {
    val: Value,
}

impl Literal {
    pub fn new(val: Value) -> Box<Self> {
        Box::new(Self { val })
    }
}
impl ASTNode for Literal {
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
