use crate::{
    ir::{marker::Expression, IRNode},
    runtime::{Interpreter, ObjectType, Value},
};

#[derive(Debug, Clone)]
pub struct CallExpression {
    name: String,
}

impl CallExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn boxed(name: &str) -> Box<Self> {
        Box::new(Self { name: name.to_owned() })
    }
}

impl IRNode for CallExpression {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        format!("{}CallExpression: {}\n", indent_str, self.name)
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let val = interpreter.global_object.get(&self.name);
        assert!(val.is_some());
        if let Value::Object(mut obj) = val.unwrap() {
            if obj.get_type() == ObjectType::Function {
                let func = obj.as_function();
                // TODO: update interpreter context to have param info here
                interpreter.run(func.body.clone())
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    }
}
impl Expression for CallExpression {}
