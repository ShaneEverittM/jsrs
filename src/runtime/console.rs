use std::{collections::HashMap, fmt};

use crate::{
    runtime::{Function, Object, Value},
    util::*,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone)]
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

impl fmt::Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_properties())
    }
}
