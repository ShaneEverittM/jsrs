use crate::runtime::{interpreter::GlobalObject, Function, Value};

#[derive(Eq, PartialEq)]
pub enum Type {
    Object,
    Global,
    Function,
    Array, // TODO
    String,
}

// TODO: properties can be much more complicated that always a key:value
pub trait Object: std::fmt::Debug + ObjectClone {
    fn put(&mut self, name: String, value: Value);

    fn get(&mut self, name: &str) -> Option<Value>;

    fn get_type(&self) -> Type;

    fn as_any(&mut self) -> &mut dyn std::any::Any;

    fn as_function(&mut self) -> &mut Function {
        assert!(self.get_type() == Type::Function);
        let any = self.as_any();
        any.downcast_mut::<Function>().unwrap()
    }

    fn as_global(&mut self) -> &mut GlobalObject {
        assert!(self.get_type() == Type::Global);
        let any = self.as_any();
        any.downcast_mut::<GlobalObject>().unwrap()
    }

    fn as_string(&mut self) -> &mut GlobalObject {
        assert!(self.get_type() == Type::Global);
        let any = self.as_any();
        any.downcast_mut::<GlobalObject>().unwrap()
    }
    
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
