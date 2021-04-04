use std::{cell::RefCell, rc::Rc};

use crate::{
    ir::{marker::Declaration, statement::Scope, IrNode},
    runtime::{exception::*, Function, Interpreter, Value},
};
use crate::util::wrap_object;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Declaration, Clone)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<String>,
    body: Scope,
}

impl FunctionDeclaration {
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

impl IrNode for FunctionDeclaration {
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
        
        interpreter.put_go_property(&self.name, Value::Object(wrap_object(function)));

        success!()
    }
}
