use std::fmt;

use crate::runtime::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Undefined,
    Object(Box<dyn Object>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => f.write_str(&n.to_string()),
            Value::Undefined => f.write_str("Undefined"),
            Value::Object(_) => {
                unimplemented!()
            }
        }
    }
}
