use std::{any::Any, collections::HashMap};

use js_object_derive::JsObject;

use crate::runtime::{Object, ObjectType, Value};

#[derive(JsObject, Clone, Debug)]
#[object_type(Console)]
pub struct Console {
    properties: HashMap<String, Value>,
}
