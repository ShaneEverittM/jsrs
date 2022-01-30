use std::{cell::RefMut, collections::HashMap};

use itertools::{EitherOrBoth, Itertools};

use crate::{
    ir::{IrNode, marker::Expression},
    runtime::{exception::*, Function, Interpreter, Value},
};
use crate::runtime::ObjectType;

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

        let parameters_and_arguments = function
            .parameters
            .iter()
            .zip_longest(self.arguments.drain(..));

        for either_or_both in parameters_and_arguments {
            match either_or_both {
                // There is an argument for this parameter, evaluate it and place it in context
                // for the function
                EitherOrBoth::Both(formal, mut actual) => {
                    context.insert(formal.clone(), actual.evaluate(interpreter)?);
                }
                // There is no argument for this parameter, substitute undefined
                EitherOrBoth::Left(formal) => {
                    context.insert(formal.clone(), Value::Undefined);
                }
                // There is an argument but no more parameters, do nothing per spec
                EitherOrBoth::Right(_) => (),
            }
        }

        if function.is_built_in() {
            let name = function.name.clone();
            // Drop here for recursion safety
            drop(function);
            interpreter.handle_built_in(&name.unwrap(), context)
        } else {
            // Drop here for recursion safety, we've already stripped the stuff required to run
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
                interpreter.global_property(&self.name)?
            }

            // Member function
            Some(object_name) => {
                // Find variable using scope resolution rules
                let val = interpreter.variable(object_name)?;

                // Check that ident resolves to an object
                if let Value::Object(obj) = val {
                    // Borrow the object we are calling a property of
                    obj.borrow_mut()
                        .get(&self.name)
                        .cloned()
                        .ok_or_else(|| ReferenceError(self.name.clone()))?
                } else {
                    return Err(TypeError("Value is not an object".to_owned()));
                }
            }
        };

        // We have the property that we want to call, check that it's an object
        if let Value::Object(func) = func {
            let func_borrow = func.borrow_mut();
            if func_borrow.get_type() == ObjectType::Function {
                // Borrow just long enough to evaluate
                let rm = RefMut::map(func_borrow, |o| o.as_function());
                return self.call(rm, interpreter);
            }
        }

        // Good exceptions
        let exception_message = match &self.member_of {
            None => format!("\'{}\' is not callable", self.name),
            Some(object) => format!(
                "Property \'{}\' of object \'{}\' is not callable",
                self.name, object
            ),
        };
        Err(TypeError(exception_message))
    }
}
