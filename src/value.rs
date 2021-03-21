use crate::object::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Undefined,
    Object(Box<dyn Object>)
}
