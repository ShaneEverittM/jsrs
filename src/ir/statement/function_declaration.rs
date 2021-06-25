use crate::{
    ir::{IrNode, marker::Expression, statement::Block},
    runtime::{exception::*, Function, Interpreter, Value},
    util::*,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct FunctionExpression {
    name: Option<String>,
    parameters: Vec<String>,
    body: Block,
}

impl FunctionExpression {
    pub fn new(name: Option<String>, parameters: Vec<String>, body: Block) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }

    pub fn boxed(name: Option<String>, parameters: Vec<String>, body: Block) -> Box<Self> {
        Box::new(Self {
            name,
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
            indent_str, self.name.as_ref().unwrap_or(&"Anonymous".into()), self.parameters
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
            interpreter.put_go_property(&self.name.as_ref().unwrap(), Value::Object(function_wrapped.clone()));
        }

        Ok(Value::Object(function_wrapped))
    }
}
