use crate::ast::Value;
use std::fmt::Debug;

pub trait Object: Debug + ObjectClone {
    fn put(&mut self, name: String, value: Value);

    fn get(&mut self, name: &str) -> Option<Value>;

    fn is_function(&self) -> bool {
        false
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any;
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Object>;
}

impl<T: 'static + Object + Clone> ObjectClone for T {
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
