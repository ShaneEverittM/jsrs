use std::{fmt, collections::HashMap};

use js_object_derive::Object;

use crate::runtime::{Value, Object};

#[derive(Object, Debug, Clone)]
#[object_type(String)]
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


impl fmt::Display for JsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_properties())
    }
}
