use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
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
        interpreter.inspect_variable(&self.object, |obj| {
            let object = match obj {
                Value::Object(o) => o,
                _ => return Err(TypeError("Expected object".to_owned())),
            };
            let property = {
                object
                    .borrow()
                    .get(&self.property)
                    .cloned()
                    .ok_or_else(|| ReferenceError(self.property.clone()))?
            };

            Ok(property)
        })
    }

    fn assign(&mut self, interpreter: &mut Interpreter, value: Value) -> Result<Value, Exception> {
        interpreter.edit_variable(&self.object, |obj| match obj {
            Value::Object(o) => {
                let mut obj_borrow = o.borrow_mut();
                let prop = obj_borrow
                    .get_mut(&self.property)
                    .ok_or_else(|| ReferenceError(self.property.clone()))?;
                *prop = value.clone();
                Ok(value)
            }
            _ => Err(TypeError("Variable is not an object".to_owned())),
        })
    }
}
