use std::collections::HashMap;

use crate::runtime::{Value, Object, ObjectType};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct JSString {
    properties: HashMap<String, Value>,
    str: String,
}

impl JSString {
    pub fn new(str: &str) -> Self {
        Self { properties: HashMap::new(), str: str.to_owned() }
    }
    pub fn boxed(str: &str) -> Box<Self> {
        Box::new(Self { properties: HashMap::new(), str: str.to_owned() })
    }
}

impl Object for JSString {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get(name).cloned()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::String
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

