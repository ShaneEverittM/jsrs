use std::any::Any;
use std::collections::HashMap;

use crate::{
    ir::statement::Scope,
    runtime::{Object, ObjectType, Value},
};

#[derive(Debug, Clone)]
pub struct Function {
    // TODO: Properties should contain the arguments, caller, callee
    //       and number of args while the function is executing as per
    //       https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions#the_arguments_object
    properties: HashMap<String, Value>,
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Scope,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Scope) -> Box<Self> {
        Box::new(Self {
            properties: HashMap::new(),
            name,
            parameters,
            body,
        })
    }
}

impl Object for Function {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get(name).cloned()
    }

    fn get_type(&self) -> ObjectType { ObjectType::Function }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
