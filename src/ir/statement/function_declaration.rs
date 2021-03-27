use crate::{
    ir::{
        IrNode,
        marker::{Declaration, Statement},
        statement::Scope,
    },
    runtime::{Function, Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<String>,
    body: Scope,
}

impl FunctionDeclaration {
    pub fn new(name: String, parameters: Vec<String>, body: Scope) -> Self {
        Self { name, parameters, body }
    }

    pub fn boxed(name: &str, parameters: Vec<String>, body: Scope) -> Box<Self> {
        Box::new(Self { name: name.to_owned(), parameters, body })
    }
}

impl IrNode for FunctionDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}FunctionDeclaration: {} | {:?}\n", indent_str, self.name, self.parameters);
        output += &self.body.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        let function = Function::new(
            self.name.clone(),
            self.parameters.clone(),
            self.body.clone(),
        );
        interpreter
            .global_object
            .put(self.name.clone(), Value::Object(function));
        None
    }
}

impl Statement for FunctionDeclaration {}

impl Declaration for FunctionDeclaration {}
