use crate::runtime::{interpreter::GlobalObject, Function, JsString, Value};

#[derive(Eq, PartialEq, Debug)]
pub enum Type {
    Object,
    Global,
    Function,
    // TODO
    Array,
    String,
}

// TODO: properties can be much more complicated that always a key:value
pub trait Object: std::fmt::Debug + ObjectClone + std::fmt::Display {
    fn put(&mut self, name: String, value: Value);

    // TODO: These two should return exceptions instead of None
    fn get(&self, name: &str) -> Option<Value>;

    fn get_mut(&mut self, name: &str) -> Option<&mut Value>;

    fn get_type(&self) -> Type;

    fn as_any(&mut self) -> &mut dyn std::any::Any;

    fn as_function(&mut self) -> &mut Function {
        assert_eq!(self.get_type(), Type::Function);
        self.as_any().downcast_mut::<Function>().unwrap()
    }

    fn as_global(&mut self) -> &mut GlobalObject {
        assert_eq!(self.get_type(), Type::Global);
        self.as_any().downcast_mut::<GlobalObject>().unwrap()
    }

    fn as_string(&mut self) -> &mut JsString {
        assert_eq!(self.get_type(), Type::String);
        self.as_any().downcast_mut::<JsString>().unwrap()
    }

    fn format_properties(&self) -> String;

}

// Helper trait to allow object-safety
pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Object>;
}

// Any static type that implements Object and Clone can be cloned into a Box
impl<T: 'static + Object + Clone> ObjectClone for T {
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

// Now we can clone a Boxed object using ObjectClone!
impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use js_object_derive::Object;
    use std::collections::HashMap;

    #[derive(Object, Clone, Debug)]
    #[object_type(Object)]
    struct SomeObject {
        properties: HashMap<String, Value>,
    }

    impl std::fmt::Display for SomeObject {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    #[derive(Object, Clone, Debug)]
    #[object_type(Object)]
    struct SomeOtherObject {
        #[properties]
        other_name: HashMap<String, Value>,
    }

    impl std::fmt::Display for SomeOtherObject {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    #[test]
    fn infer_properties() {
        let mut o = SomeObject {
            properties: HashMap::new(),
        };

        o.put("Prop".to_owned(), Value::Boolean(true));

        assert_eq!(o.get("Prop"), Some(Value::Boolean(true)));
    }

    #[test]
    fn annotate_properties() {
        let mut o = SomeObject {
            properties: HashMap::new(),
        };

        o.put("Prop".to_owned(), Value::Boolean(true));

        assert_eq!(o.get("Prop"), Some(Value::Boolean(true)));
    }
}
