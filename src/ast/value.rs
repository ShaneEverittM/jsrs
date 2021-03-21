use crate::runtime::Object;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Undefined,
    Object(Box<dyn Object>)
}
