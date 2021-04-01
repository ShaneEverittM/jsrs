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
        format!("{}MemberExpression: {}\n", indent_str, self.property)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.edit_variable(&self.object, |obj| {
            let object = match obj {
                Value::Object(o) => o,
                _ => return Err(Exception::TypeError("Expected object".to_owned())),
            };
            let property = object
                .borrow()
                .get(&self.property)
                .ok_or_else(|| Exception::ReferenceError(self.property.clone()))?;

            Ok(property)
        })
    }
}

impl Expression for MemberExpression {}
