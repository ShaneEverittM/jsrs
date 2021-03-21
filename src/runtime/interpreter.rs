use std::any::Any;
use std::collections::HashMap;

use crate::{
    ast::statement::BlockStatement,
    runtime::{Object, Value},
};

pub struct Interpreter {
    pub global_object: Box<dyn Object>,
    scope_stack: Vec<BlockStatement>,
}

#[derive(Debug, Clone)]
pub struct GlobalObject {
    properties: HashMap<String, Box<Value>>,
}

impl GlobalObject {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}

impl Object for GlobalObject {
    fn put(&mut self, name: String, value: Value) {
        self.properties.insert(name, Box::new(value));
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get_mut(name).map(|b| *b.clone())
    }

    fn is_function(&self) -> bool {
        true
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            global_object: Box::new(GlobalObject::new()),
            scope_stack: Vec::new(),
        }
    }
}

impl Interpreter {
    pub fn run(&mut self, mut block: Box<BlockStatement>) -> Value {
        // TODO: is this clone avoidable, and if not is it really really bad?
        self.enter_scope(*block.clone());

        let mut last_value = Value::Undefined;

        for node in block.children.iter_mut() {
            last_value = node.evaluate(self);
        }

        self.pop_scope();

        last_value
    }

    fn enter_scope(&mut self, scope: BlockStatement) {
        self.scope_stack.push(scope);
    }

    fn pop_scope(&mut self) {
        assert!(self.scope_stack.last().is_some());
        self.scope_stack.pop();
    }
}
