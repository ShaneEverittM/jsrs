use itertools::{EitherOrBoth, Itertools};

use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{Interpreter, ObjectType, Value},
};

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
        Box::new(Self {
            name: name.to_owned(),
            arguments,
        })
    }
}

/// A call expression can be evaluated by looking up the function in the current scope,
/// binding parameters, and executing the body.
impl IrNode for CallExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}CallExpression: {}\n", indent_str, self.name)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        // TODO: Currently only top level function are supported
        let val = interpreter.global_object.get(&self.name);

        // Look for symbol in global object
        if let Value::Object(mut obj) = val.expect("Could not find function") {
            // Verify it is in fact a function (or later at least callable)
            if obj.get_type() == ObjectType::Function {
                let func = obj.as_function();

                // bind formal parameters to actual parameters (thanks Klefstad)
                let context = func
                    .parameters
                    .iter()
                    .zip_longest(self.arguments.drain(..))
                    .map(|eob| match eob {
                        EitherOrBoth::Both(formal, mut actual) => (
                            formal.clone(),
                            actual.evaluate(interpreter).unwrap_or_default(),
                        ),
                        EitherOrBoth::Left(formal) => (formal.clone(), Value::Undefined),
                        EitherOrBoth::Right(_) => panic!("Too many arguments"),
                    })
                    .collect();

                interpreter.run_with(func.body.clone(), context)
            } else {
                unimplemented!("Only current callable type is a function")
            }
        } else {
            unimplemented!("Cannot call value of non-function type")
        }
    }
}

impl Expression for CallExpression {}
