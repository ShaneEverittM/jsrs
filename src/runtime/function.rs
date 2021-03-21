use std::any::Any;
use std::collections::HashMap;

use crate::{
    ast::statement::BlockStatement,
    runtime::{Object, Value},
};

#[derive(Debug, Clone)]
pub struct Function {
    // TODO: Properties should contain the arguments, caller, callee
    //       and number of args while the function is executing as per
    //       https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions#the_arguments_object
    properties: HashMap<String, Box<Value>>,
    pub name: String,
    pub body: Box<BlockStatement>,
}

impl Function {
    pub fn new(name: String, body: Box<BlockStatement>) -> Box<Self> {
        Box::new(Self {
            properties: HashMap::new(),
            name,
            body,
        })
    }
}

impl Object for Function {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, Box::new(value));
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get_mut(name).map(|b| *b.clone())
    }

    fn is_function(&self) -> bool {
        true
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
