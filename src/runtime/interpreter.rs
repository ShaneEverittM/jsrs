use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    ir::statement::{Block, BlockType},
    runtime::{Console, exception::*, Object, Value},
    util::*,
};

/// The global object containing top level definitions of built-in functions
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone, Default)]
#[object_type(Global)]
pub struct GlobalObject {
    properties: HashMap<String, Value>,
}

impl fmt::Display for GlobalObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&Object::format_properties(self))
    }
}

/// The Interpreter in charge of everything, handles all evaluation, printing, built-in functions,
/// exceptions and (eventually) memory management.
pub struct Interpreter {
    // The global object, also known as "window" or "globalThis", contains some top level defs
    global_object: Rc<RefCell<Box<dyn Object>>>,

    // The stack of scope for variable resolution
    scope_stack: Vec<HashMap<String, Value>>,

    // Flag indicating if the interpreter should break out of its current context
    should_break: bool,

    // Flag indicating if the interpreter should return from the current function
    should_return: bool,

    // Register to hold return values so that expression values can "skip" up arbitrary levels
    return_register: Option<Value>,

    // A counter indicating how many times it was requested of the interpreter to not treat
    // function declarations as global declarations, useful for functions as properties or variables
    declaration_suppression_counter: usize,

    // Just a buffer to hold the initial scope with its useful things from construction
    // until first call to interpret, at which point it gets moved to the scope stack
    // as the bottom
    _gs_slot: Option<HashMap<String, Value>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        // Create the global object
        let global_object = Rc::new(RefCell::new(
            Box::new(GlobalObject::default()) as Box<dyn Object>
        ));

        // Create the root scope
        let mut global_scope = HashMap::new();

        // Alias global object under these names
        global_scope.insert(
            "globalThis".to_owned(),
            Value::Object(Rc::clone(&global_object)),
        );
        global_scope.insert(
            "window".to_owned(),
            Value::Object(Rc::clone(&global_object)),
        );

        // Add base functions to the global object
        Self::populate_built_ins(Rc::clone(&global_object));

        Self {
            global_object,
            scope_stack: Vec::new(),
            should_break: false,
            should_return: false,
            return_register: None,
            declaration_suppression_counter: 0,
            _gs_slot: Some(global_scope),
        }
    }

    pub fn dump_state(&self) {
        let mut go_ref = self.global_object.borrow_mut();
        let go = go_ref.as_global();
        println!("Global Object State");
        for (key, value) in go.properties.iter() {
            println!("{}: {}", key, value);
        }
        println!("End Global Object State")
    }

    pub fn run(&mut self, block: Block) -> Result<Value, Exception> {
        let global_scope = self._gs_slot.take().unwrap();
        match self.run_with(block, global_scope) {
            Ok(value) => Ok(value),
            Err(e) => {
                self.handle_exception(e.clone());
                Err(e)
            }
        }
    }

    pub fn evaluate_scope(&mut self, block: Block) -> Result<Value, Exception> {
        self.run_with(block, HashMap::new())
    }

    pub fn run_with(
        &mut self,
        mut block: Block,
        context: HashMap<String, Value>,
    ) -> Result<Value, Exception> {
        self.enter_scope(context);

        let mut last_value = Value::Undefined;

        for node in block.children.iter_mut() {
            last_value = node.evaluate(self)?;

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
                if block.get_type() == BlockType::Function {
                    self.clear_return();
                    last_value = self.return_register.take().unwrap_or_default();
                }
                break;
            }
        }

        self.leave_scope();

        Ok(last_value)
    }

    pub fn handle_built_in(
        &mut self,
        name: &str,
        context: HashMap<String, Value>,
    ) -> Result<Value, Exception> {
        match name {
            "console_log" => {
                let expr = context
                    .get("expr")
                    .ok_or_else(|| ReferenceError("expression".to_owned()))?;
                println!("{}", expr);
                success!()
            }
            "other" => success!(),
            _ => success!(),
        }
    }

    pub fn at_global(&self) -> bool {
        self.scope_stack.len() == 1
    }

    pub fn add_variable(&mut self, key: String, value: Value) {
        self.scope_stack.last_mut().unwrap().insert(key, value);
    }

    pub fn get_go_property(&mut self, name: &str) -> Result<Value, Exception> {
        self.global_object
            .borrow_mut()
            .get(name)
            .ok_or_else(|| Exception::ReferenceError(name.to_owned()))
    }

    pub fn put_go_property(&mut self, name: &str, property: Value) {
        self.global_object
            .borrow_mut()
            .put(name.to_owned(), property)
    }

    /// Get the value of a variable with name `name`, using scope resolution.
    pub fn get_variable(&mut self, name: &str) -> Result<Value, Exception> {
        match self.resolve_variable(name) {
            None => match self.global_object.borrow().get(name) {
                None => Err(ReferenceError(name.to_owned())),
                Some(v) => Ok(v),
            },
            Some(v) => Ok(v),
        }
    }

    /// Finds the a variable given `name`, and applies the closure `edit` to it.
    pub fn edit_variable<F>(&mut self, name: &str, edit: F) -> Result<Value, Exception>
    where
        F: FnOnce(&mut Value) -> Result<Value, Exception>,
    {
        // Look up in normal scope stack
        match self.resolve_variable_mut(name) {
            // If not found, check if it is a property of the global object
            None => match self.global_object.borrow_mut().get_mut(name) {
                None => Err(ReferenceError(name.to_owned())),
                Some(v) => edit(v),
            },
            Some(v) => edit(v),
        }
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

    pub fn suppress_declarations(&mut self) {
        self.declaration_suppression_counter += 1
    }
    pub fn clear_suppress_declarations(&mut self) {
        self.declaration_suppression_counter -= 1;
    }
    pub fn should_suppress_declarations(&self) -> bool {
        self.declaration_suppression_counter > 0
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

    fn resolve_variable_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.scope_stack
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(name))
    }
    fn resolve_variable(&self, name: &str) -> Option<Value> {
        self.scope_stack
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    fn populate_built_ins(global_object: Rc<RefCell<Box<dyn Object>>>) {
        global_object.borrow_mut().put(
            "console".to_owned(),
            Value::Object(wrap_object(Console::boxed())),
        );
    }

    fn handle_exception(&mut self, _exception: Exception) {
        #[cfg(not(feature = "suppress_exceptions"))]
        eprintln!("{}", _exception.to_string());
    }
}
