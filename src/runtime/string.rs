use std::{collections::HashMap, fmt};

use crate::runtime::{Object, Value};

#[allow(unused)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone)]
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
