use crate::runtime::Value;

pub trait Object: std::fmt::Debug + ObjectClone {
    fn put(&mut self, name: String, value: Value);

    fn get(&mut self, name: &str) -> Option<Value>;

    fn is_function(&self) -> bool {
        false
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any;
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
