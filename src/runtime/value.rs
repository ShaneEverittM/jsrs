use std::fmt;

use crate::runtime::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Undefined,
    Boolean(bool),
    Object(Box<dyn Object>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => f.write_str(&n.to_string()),
            Value::Undefined => f.write_str("Undefined"),
            Value::Boolean(b) => f.write_str(&b.to_string()),
            Value::Object(_) => {
                unimplemented!()
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            // TODO: Is undefined == undefined? What about === ?
            (Value::Undefined, Value::Undefined) => true,
            (Value::Object(_), Value::Object(_)) => {
                // TODO: Two boxes can't point to the same location, that's aliasing.
                //  So, how can you have object equality? May have to go unsafe with raw
                //  pointers...
                false
            },
            _ => false,
        }
    }
}
