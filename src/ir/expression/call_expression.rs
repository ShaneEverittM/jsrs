use std::cell::RefMut;

use itertools::{EitherOrBoth, Itertools};

use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{Interpreter, Value},
};
use crate::runtime::{Exception, Function};

#[derive(Debug, Clone)]
pub struct CallExpression {
    member_of: Option<String>,
    name: String,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(name: String, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { member_of: None, name, arguments }
    }

    pub fn boxed(name: &str, arguments: Vec<Box<dyn Expression>>) -> Box<Self> {
        Box::new(Self {
            member_of: None,
            name: name.to_owned(),
            arguments,
        })
    }

    pub fn boxed_member(member_of: &str, name: &str, arguments: Vec<Box<dyn Expression>>) -> Box<Self> {
        Box::new(Self {
            member_of: Some(member_of.to_owned()),
            name: name.to_owned(),
            arguments,
        })
    }


    fn call_internal(&mut self, function: RefMut<Function>, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let block = function.body.clone();

        // bind formal parameters to actual parameters (thanks Klefstad)
        let context = function
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

        drop(function);

        interpreter.run_with(block, context)
    }
}

impl IrNode for CallExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}CallExpression: {}\n", indent_str, self.name)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        let func = match self.member_of.as_ref() {
            // Free function
            None => {
                // Get function as a property of the global object
                interpreter.get_go_property(&self.name).unwrap()
            }

            // Member function
            Some(object_name) => {
                // Find variable using scope resolution rules
                let val = interpreter.resolve_variable(object_name).expect("Cannot find function");

                // Check that ident resolves to an object
                if let Value::Object(obj) = val {
                    // Borrow the object we are calling a property of
                    obj.borrow_mut().get(&self.name).expect("Object has no function with given name")
                } else {
                    panic!("Identifier is not an object")
                }
            }
        };

        // We have the property that we want to call, check that it's an object
        if let Value::Object(func) = func {
            // Borrow just long enough to evaluate
            let rm = RefMut::map(func.borrow_mut(), |o| o.as_function());
            self.call_internal(rm, interpreter)
        } else {
            unimplemented!("Only current callable type is a function")
        }
    }
}

impl Expression for CallExpression {}
