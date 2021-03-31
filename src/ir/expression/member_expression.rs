use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct MemberExpression {
    object: String,
    property: String,
}

impl MemberExpression {
    pub fn boxed(object: String, property: String) -> Box<Self> {
        Box::new(Self { object, property })
    }
}

impl IrNode for MemberExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}MemberCallExpression: {}\n", indent_str, self.property)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.get_variable(&self.object)
    }
}

impl Expression for MemberExpression {}
