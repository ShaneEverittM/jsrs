use std::{collections::HashMap, fmt};

use crate::runtime::{Object, Value};

// TODO: make a derive macro that can implement object for a type by annotating the
//  field with a map interface
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone)]
#[object_type(Object)]
pub struct LiteralObject {
    properties: HashMap<String, Value>,
}

impl LiteralObject {
    pub fn new(properties: HashMap<String, Value>) -> Self {
        Self {properties}
    }
    pub fn boxed(properties: HashMap<String, Value>) -> Box<Self> {
        Box::new(Self::new(properties))
    }
}

impl fmt::Display for LiteralObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_properties())
    }
}
