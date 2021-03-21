use std::any::Any;
use std::collections::HashMap;

use super::Object;
use crate::ast::{Block, Value};

#[derive(Debug, Clone)]
pub struct Function {
    properties: HashMap<String, Box<Value>>,
    pub name: String,
    pub body: Box<Block>,
}

impl Function {
    pub fn new(name: String, body: Box<Block>) -> Box<Self> {
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
