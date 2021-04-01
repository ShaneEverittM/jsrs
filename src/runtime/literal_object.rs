use std::collections::HashMap;

use js_object_derive::Object;

use crate::runtime::Value;

// TODO: make a derive macro that can implement object for a type by annotating the
//  field with a map interface
#[derive(Object, Debug, Clone)]
#[object_type(Object)]
pub struct LiteralObject {
    properties: HashMap<String, Value>,
}
