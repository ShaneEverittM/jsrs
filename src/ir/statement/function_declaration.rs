use crate::{
    ir::{marker::Expression, statement::Scope, IrNode},
    runtime::{exception::*, Function, Interpreter, Value},
    util::*,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct FunctionExpression {
    name: String,
    parameters: Vec<String>,
    body: Scope,
}

impl FunctionExpression {
    pub fn new(name: String, parameters: Vec<String>, body: Scope) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub fn boxed(name: &str, parameters: Vec<String>, body: Scope) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            parameters,
            body,
        })
    }
}

impl IrNode for FunctionExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!(
            "{}FunctionDeclaration: {} | {:?}\n",
            indent_str, self.name, self.parameters
        );
        output += &self.body.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let function = Function::new(
            self.name.clone(),
            self.parameters.clone(),
            self.body.clone(),
        );

        let function_wrapped = wrap_object(function);

        if !interpreter.should_suppress_declarations() {
            interpreter.put_go_property(&self.name, Value::Object(function_wrapped.clone()));
        }

        Ok(Value::Object(function_wrapped))
    }
}
