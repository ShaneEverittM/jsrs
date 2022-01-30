use std::{collections::HashMap, fmt};

use crate::{
    runtime::{Function, Object, Value},
    util::*,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone)]
#[object_type(Object)]
pub struct Console {
    #[properties]
    properties: HashMap<String, Value>,
}

impl Console {
    pub fn new() -> Self {
        let mut properties = HashMap::new();

        let log_function_built_in =
            Function::built_in(Some("console_log".to_string()), vec!["expr".to_string()]);

        let log_function_object = log_function_built_in.value();

        properties.insert("log".to_string(), log_function_object);

        Self { properties }
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_properties())
    }
}
