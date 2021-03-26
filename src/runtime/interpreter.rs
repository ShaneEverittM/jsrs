use std::any::Any;
use std::collections::HashMap;

use crate::{
    ir::statement::Scope,
    runtime::{Object, ObjectType, Value},
};

pub struct Interpreter {
    pub global_object: Box<dyn Object>,
    pub scope_stack: Vec<HashMap<String, Value>>,
    should_break: bool,
}

#[derive(Debug, Clone)]
pub struct GlobalObject {
    properties: HashMap<String, Value>,
}

impl Default for GlobalObject {
    fn default() -> Self {
        GlobalObject::new()
    }
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
        self.properties.insert(name, value);
    }

    fn get(&mut self, name: &str) -> Option<Value> {
        self.properties.get(name).cloned()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Global
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            global_object: Box::new(GlobalObject::new()),
            // TODO: Put global object alias here, like "window" or "globalThis"
            scope_stack: Vec::new(),
            should_break: false,
        }
    }
}

impl Interpreter {
    pub fn run(&mut self, mut block: Scope) -> Value {
        self.enter_scope(HashMap::new());

        let mut last_value = Value::Undefined;

        for node in block.children.iter_mut() {
            last_value = node.evaluate(self);
            /*
            Break out of evaluating block, but don't clear, since we are probably
            running inside Loop::evaluate() and it needs to stop looping. Two rust breaks
            are needed to get one JS break, one to stop evaluating the block, another to stop
            iterating
            */
            if self.should_break {
                break;
            }
        }

        self.pop_scope();

        last_value
    }

    pub fn resolve_variable(&mut self, name: &str) -> Option<&mut Value> {
        for scope in self.scope_stack.iter_mut().rev() {
            match scope.get_mut(name) {
                None => continue,
                Some(v) => return Some(v),
            }
        }
        None
    }

    pub fn notify_break(&mut self) {
        self.should_break = true;
    }

    pub fn clear_break(&mut self) {
        self.should_break = false;
    }

    pub fn broke(&self) -> bool {
        self.should_break
    }

    fn enter_scope(&mut self, scope: HashMap<String, Value>) {
        self.scope_stack.push(scope);
    }

    fn pop_scope(&mut self) {
        assert!(self.scope_stack.last().is_some());
        self.scope_stack.pop();
    }
}
