use std::{any::Any, collections::HashMap};

use crate::runtime::{Object, ObjectType, Value};

#[derive(Debug, Clone)]
pub struct JsString {
    properties: HashMap<String, Value>,
    str: String,
}

impl JsString {
    pub fn new(str: &str) -> Self {
        Self {
            properties: HashMap::new(),
            str: str.to_owned(),
        }
    }
    pub fn boxed(str: &str) -> Box<Self> {
        Box::new(Self {
            properties: HashMap::new(),
            str: str.to_owned(),
        })
    }
}

impl Object for JsString {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<Value> {
        self.properties.get(name).cloned()
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.properties.get_mut(name)
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::String
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
