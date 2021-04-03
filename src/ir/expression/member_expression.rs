use crate::{
    ir::{marker::Expression, IrNode},
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

    /// Applies the given closure to the value that is yielded by this expression.
    ///
    /// Essentially this is one step more than Interpreter::edit_variable(). First we use said
    /// function to enter a context with mutable access to the object we want to edit, then we
    /// apply the given closure to its property.
    fn edit_lvalue(
        &mut self,
        interpreter: &mut Interpreter,
        edit: Box<dyn FnOnce(&mut Value) -> Result<Value, Exception>>,
    ) -> Result<Value, Exception> {
        interpreter.edit_variable(&self.object, |obj| match obj {
            Value::Object(o) => {
                let mut obj_borrow = o.borrow_mut();
                let prop = obj_borrow
                    .get_mut(&self.property)
                    .ok_or_else(|| Exception::ReferenceError(self.property.clone()))?;
                edit(prop)
            }
            _ => Err(Exception::TypeError("Variable is not an object".to_owned())),
        })
    }
}
