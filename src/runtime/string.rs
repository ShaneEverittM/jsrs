use std::collections::HashMap;

use js_object_derive::Object;

use crate::runtime::Value;

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
