use crate::{
    ir::{
        marker::{Expression, Statement},
        IrNode,
    },
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Option<Box<dyn Expression>>,
}

impl VariableDeclaration {
    pub fn new(name: &str, value: Box<dyn Expression>) -> Self {
        Self {
            name: name.to_owned(),
            value: Some(value),
        }
    }

    pub fn boxed(name: &str, value: Box<dyn Expression>) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            value: Some(value),
        })
    }
}

impl IrNode for VariableDeclaration {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}VariableDeclaration: {}\n", indent_str, self.name);
        if let Some(value) = self.value.as_ref() {
            output += &value.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let value = self
            .value
            .as_mut()
            .map(|v| v.evaluate(interpreter).unwrap_or_default())
            .unwrap_or(Value::Undefined);

        interpreter.add_variable(self.name.clone(), value);

        success!()
    }
}
