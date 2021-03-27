use std::any::Any;
use std::collections::HashMap;

use crate::{
    ir::statement::Scope,
    runtime::{Object, ObjectType, Value},
};
use crate::ir::statement::ScopeType;

pub struct Interpreter {
    pub global_object: Box<dyn Object>,
    pub scope_stack: Vec<HashMap<String, Value>>,
    should_break: bool,
    should_return: bool,
    return_register: Option<Value>,
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
            should_return: false,
            return_register: None,
        }
    }
}

impl Interpreter {
    pub fn run_with(&mut self, mut block: Scope, context: HashMap<String, Value>) -> Option<Value> {
        self.enter_scope(context);

        let mut last_value = None;

        for node in block.children.iter_mut() {
            if let Some(val) = node.evaluate(self) {
                last_value = Some(val)
            }

            /*
            Break out of evaluating block, but don't clear, since we are probably
            running inside Loop::evaluate() and it needs to stop looping. Two rust breaks
            are needed to get one JS break, one to stop evaluating the block, another to stop
            iterating
            */
            if self.should_break {
                break;
            }

            /*
            Keep breaking out of scopes, which will put us right after the call to evaluate
            above, until we get to a function, meaning we found the function from which we
            should return. Then clear return flag, and propagate value.
            */
            if self.should_return {
                if *block.get_type() == ScopeType::Function {
                    self.clear_return();
                    last_value = self.return_register.take();
                }
                break;
            }
        }

        self.pop_scope();

        last_value
    }
    pub fn run(&mut self, block: Scope) -> Option<Value> {
        self.run_with(block, HashMap::new())
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

    pub fn notify_return(&mut self) {
        self.should_return = true;
    }

    pub fn clear_return(&mut self) {
        self.should_return = false;
    }

    pub fn returned(&self) -> bool {
        self.should_return
    }

    pub fn set_return_val(&mut self, val: Value) {
        self.return_register = Some(val)
    }

    fn enter_scope(&mut self, scope: HashMap<String, Value>) {
        self.scope_stack.push(scope);
    }

    fn pop_scope(&mut self) {
        assert!(self.scope_stack.last().is_some());
        self.scope_stack.pop();
    }
}
