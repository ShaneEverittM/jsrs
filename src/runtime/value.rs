use std::{cell::RefCell, fmt, rc::Rc};

use crate::runtime::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Undefined,
    Boolean(bool),
    String(String),
    Object(Rc<RefCell<Box<dyn Object>>>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Undefined
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => f.write_str(&n.to_string()),
            Value::Undefined => f.write_str("Undefined"),
            Value::Boolean(b) => f.write_str(&b.to_string()),
            Value::String(s) => f.write_str(&s),
            Value::Object(o) => {
                let s = format!("{}", o.borrow());
                f.write_str(&s)
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            // ?: Is undefined == undefined? What about === ?
            (Value::Undefined, Value::Undefined) => true,
            (Value::Object(_), Value::Object(_)) => {
                // ?: Two boxes can't point to the same location, that's aliasing.
                //  So, how can you have object equality? May have to go unsafe with raw
                //  pointers...
                false
            }
            _ => false,
        }
    }
}
