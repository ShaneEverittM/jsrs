use std::{cell::RefMut, collections::HashMap};

use itertools::{EitherOrBoth, Itertools};

use crate::{
    ir::{marker::Expression, IrNode},
    runtime::{exception::*, Function, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Expression, Clone)]
pub struct CallExpression {
    member_of: Option<String>,
    name: String,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(name: String, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self {
            member_of: None,
            name,
            arguments,
        }
    }

    pub fn boxed(name: &str, arguments: Vec<Box<dyn Expression>>) -> Box<Self> {
        Box::new(Self {
            member_of: None,
            name: name.to_owned(),
            arguments,
        })
    }

    pub fn boxed_member(
        member_of: &str,
        name: &str,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Box<Self> {
        Box::new(Self {
            member_of: Some(member_of.to_owned()),
            name: name.to_owned(),
            arguments,
        })
    }

    fn call(
        &mut self,
        function: RefMut<Function>,
        interpreter: &mut Interpreter,
    ) -> Result<Value, Exception> {
        let block = function.body.clone();

        // bind formal parameters to actual parameters (thanks Klefstad)
        let mut context = HashMap::new();
        for eob in function
            .parameters
            .iter()
            .zip_longest(self.arguments.drain(..))
        {
            match eob {
                EitherOrBoth::Both(formal, mut actual) => {
                    context.insert(formal.clone(), actual.evaluate(interpreter)?);
                }
                EitherOrBoth::Left(formal) => {
                    context.insert(formal.clone(), Value::Undefined);
                }
                EitherOrBoth::Right(_) => panic!("Too many arguments"),
            }
        }

        if function.is_built_in() {
            let name = function.name.clone();
            drop(function);
            interpreter.handle_built_in(&name, context)
        } else {
            drop(function);
            interpreter.run_with(block, context)
        }
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
                interpreter.get_go_property(&self.name)?
            }

            // Member function
            Some(object_name) => {
                // Find variable using scope resolution rules
                let val = interpreter.get_variable(object_name)?;

                // Check that ident resolves to an object
                if let Value::Object(obj) = val {
                    // Borrow the object we are calling a property of
                    obj.borrow_mut()
                        .get(&self.name)
                        .ok_or_else(|| ReferenceError(self.name.clone()))?
                } else {
                    return Err(TypeError("Value is not an object".to_owned()));
                }
            }
        };

        // We have the property that we want to call, check that it's an object
        if let Value::Object(func) = func {
            // Borrow just long enough to evaluate
            let rm = RefMut::map(func.borrow_mut(), |o| o.as_function());
            self.call(rm, interpreter)
        } else {
            Err(TypeError("Type is not callable".to_owned()))
        }
    }
}
