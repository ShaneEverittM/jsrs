use std::{any::Any, collections::HashMap};

use crate::runtime::{Object, ObjectType, Value};

// TODO: make a derive macro that can implement object for a type by annotating the
//  field with a map interface
#[derive(Debug, Clone)]
pub struct LiteralObject {
    properties: HashMap<String, Value>,
}

impl Object for LiteralObject {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get(name).cloned()
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.properties.get_mut(name)
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Object
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
