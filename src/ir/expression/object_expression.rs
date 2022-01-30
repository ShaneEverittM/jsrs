use std::collections::HashMap;

use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{exception::*, Interpreter, Value},
    util::*,
};
use crate::prelude::LiteralObject;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct ObjectExpression {
    keys: Vec<String>,
    values: Vec<Box<dyn Expression>>,
}

impl ObjectExpression {
    pub fn boxed(keys: Vec<String>, values: Vec<Box<dyn Expression>>) -> Box<Self> {
        Box::new(Self { keys, values })
    }
}

impl IrNode for ObjectExpression {
    fn dump(&self, indent: u32) -> String {
        let indent = crate::util::make_indent(indent);
        let output = format!("{}ObjectExpression\n", indent);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let mut props = HashMap::new();
        let keys_and_values = self.keys.iter_mut().zip(self.values.iter_mut());

        // While evaluating the values for the properties, any function declarations should
        // not put themselves in top level scope
        // TODO: Is this the best way? Hard to get them there otherwise since main interpreter
        //  eval loop is blind to what is being evaluated
        interpreter.suppress_declarations();
        for (key, value_expr) in keys_and_values {
            let value = value_expr.evaluate(interpreter)?;
            props.insert(key.clone(), value);
        }
        interpreter.allow_declarations();

        Ok(Value::Object(LiteralObject::new(props).bundle()))
    }
}
