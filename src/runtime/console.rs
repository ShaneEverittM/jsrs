use std::collections::HashMap;

use js_object_derive::Object;

use crate::{
    runtime::{Function, Value},
    util::*,
};

#[derive(Object, Clone, Debug)]
#[object_type(Object)]
pub struct Console {
    properties: HashMap<String, Value>,
}

impl Console {
    pub fn boxed() -> Box<Self> {
        let mut properties = HashMap::new();

        let log_function_built_in =
            Function::built_in("console_log".to_owned(), vec!["expr".to_owned()]);

        let log_function_object = wrap_object(log_function_built_in);

        properties.insert("log".to_owned(), Value::Object(log_function_object));

        Box::new(Self { properties })
    }
}
