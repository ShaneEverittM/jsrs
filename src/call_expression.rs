use crate::ast_node::ASTNode;
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::marker::Expression;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct CallExpression {
    name: String,
}

impl CallExpression {
    pub fn new(name: String) -> Box<Self> {
        Box::new(Self { name })
    }
}

impl ASTNode for CallExpression {
    fn dump(&self) -> String {
        unimplemented!()
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        let val = interpreter.global_object.get(&self.name);
        assert!(val.is_some());
        if let Value::Object(mut obj) = val.unwrap() {
            if obj.is_function() {
                let func = obj.as_any().downcast_mut::<Function>().unwrap();
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
