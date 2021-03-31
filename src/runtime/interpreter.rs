use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    ir::statement::Scope,
    runtime::{Object, ObjectType, Value},
};
use crate::ir::statement::ScopeType;

pub struct Interpreter {
    global_object: Rc<RefCell<Box<dyn Object>>>,
    scope_stack: Vec<HashMap<String, Value>>,
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

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.properties.get_mut(name)
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
        let go = Rc::new(RefCell::new(
            Box::new(GlobalObject::new()) as Box<dyn Object>
        ));
        let mut global_scope = HashMap::new();
        global_scope.insert("globalThis".to_owned(), Value::Object(go.clone()));

        Self {
            global_object: go,
            scope_stack: vec![global_scope],
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

        self.leave_scope();

        last_value
    }
    pub fn run(&mut self, block: Scope) -> Option<Value> {
        self.run_with(block, HashMap::new())
    }

    pub fn resolve_variable(&mut self, name: &str) -> Option<&mut Value> {
        self.scope_stack
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }

    pub fn add_variable(&mut self, key: String, value: Value) {
        self.scope_stack.last_mut().unwrap().insert(key, value);
    }

    pub fn get_go_property(&mut self, name: &str) -> Option<Value> {
        self.global_object.borrow_mut().get(name)
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

    fn leave_scope(&mut self) {
        assert!(self.scope_stack.last().is_some());
        self.scope_stack.pop();
    }
}
