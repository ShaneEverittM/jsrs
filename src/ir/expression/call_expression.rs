use crate::{
    ir::{IRNode, marker::Expression},
    runtime::{Interpreter, ObjectType, Value},
};
use crate::ir::expression::Literal;

#[derive(Debug, Clone)]
pub struct CallExpression {
    name: String,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(name: String, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { name, arguments }
    }

    pub fn boxed(name: &str, arguments: Vec<Box<dyn Expression>>) -> Box<Self> {
        Box::new(Self { name: name.to_owned(), arguments })
    }
}

impl IRNode for CallExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}CallExpression: {}\n", indent_str, self.name)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        let val = interpreter.global_object.get(&self.name);
        assert!(val.is_some());
        if let Value::Object(mut obj) = val.unwrap() {
            if obj.get_type() == ObjectType::Function {
                let func = obj.as_function();
                // bind formal parameters to actual parameters (thanks Klefstad)
                let missing = func.parameters.len() - self.arguments.len();
                for _ in 0..missing {
                    self.arguments.push(Literal::boxed(Value::Undefined) as Box<dyn Expression>);
                }
                let context = func.parameters
                    .iter()
                    .zip(self.arguments.drain(..))
                    .map(|(formal, mut actual)| {
                        (formal.clone(), actual.evaluate(interpreter).unwrap_or(Value::Undefined))
                    }).collect();
                interpreter.run_with(func.body.clone(), context)
            } else {
                unimplemented!("Only current callable type is a function")
            }
        } else {
            unimplemented!("Cannot resolve callable name")
        }
    }
}

impl Expression for CallExpression {}
