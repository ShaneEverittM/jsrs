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
    break_flag: bool,

    // Flag indicating if the interpreter should return from the current function
    return_flag: bool,

    // Register to hold return values so that expression values can "skip" up arbitrary levels
    return_register: Option<Value>,

    // A counter indicating how many times it was requested of the interpreter to not treat
    // function declarations as global declarations, useful for functions as properties or variables
    declaration_suppression_counter: usize,

    // Just a buffer to hold the initial scope with its useful things from construction
    // until first call to interpret, at which point it gets moved to the scope stack
    // as the bottom
    base_global_scope: HashMap<String, Value>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let global_object = GlobalObject::bundled();

        // Create the top level scope
        let mut global_scope = HashMap::new();

        // Alias global object under these names
        global_scope.insert(
            String::from("globalThis"),
            Value::Object(Rc::clone(&global_object)),
        );
        global_scope.insert(
            String::from("window"),
            Value::Object(Rc::clone(&global_object)),
        );

        // Add base functions to the global object
        Self::populate_built_ins(Rc::clone(&global_object));

        Self {
            global_object,
            scope_stack: Vec::new(),
            break_flag: false,
            return_flag: false,
            return_register: None,
            declaration_suppression_counter: 0,
            base_global_scope: global_scope,
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
        match self.run_with(block, self.base_global_scope.clone()) {
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
        // Running a block, so create a scope for it
        self.enter_scope(context);

        // Every node in the AST may produce a value if it is Expression
        let mut last_value = Value::Undefined;

        // Evaluate all of the children of this node
        for node in block.children.iter_mut() {
            last_value = node.evaluate(self)?;

            /*
            Break out of evaluating block, but don't clear, since we are probably
            running inside Loop::evaluate() and it needs to stop looping. Two rust breaks
            are needed to get one JS break, one to stop evaluating the block, another to stop
            iterating.
            */
            if self.break_flag {
                break;
            }

            /*
            Keep breaking out of scopes, which will put us right after the call to evaluate
            above, until we get to a function, meaning we found the function from which we
            should return. Then clear return flag, and propagate value.
            */
            if self.return_flag {
                if block.get_type() == BlockType::Function {
                    self.clear_return();
                    last_value = self.return_register.take().unwrap_or_default();
                }
                break;
            }
        }

        // Done with the block, leave scope
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
            _ => success!(),
        }
    }

    pub fn at_global(&self) -> bool {
        self.scope_stack.len() == 1
    }

    pub fn add_variable(&mut self, key: String, value: Value) {
        self.scope_stack.last_mut().unwrap().insert(key, value);
    }

    pub fn global_property(&self, name: &str) -> Result<Value, Exception> {
        self.global_object
            .borrow()
            .get(name)
            .cloned()
            .ok_or_else(|| Exception::ReferenceError(name.to_owned()))
    }

    pub fn set_global_property(&mut self, name: &str, property: Value) {
        self.global_object.borrow_mut().put(name, property)
    }

    /// Get the value of a variable with name `name`, using scope resolution.
    pub fn variable(&mut self, name: &str) -> Result<Value, Exception> {
        let x = self.global_object.borrow();
        match self.resolve_variable(name).cloned() {
            None => match x.get(name).cloned() {
                None => Err(ReferenceError(name.to_owned())),
                Some(v) => Ok(v),
            },
            Some(v) => Ok(v),
        }
    }

    /// Finds the variable with the given `name`, and applies the closure `edit` to it.
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

    ///
    /// Allows complicated inspection of a variable of the given name in the current scope.
    ///
    /// Useful since it is difficult to return a reference into the global object
    /// without violating ownership rules, but give the ergonomics of having the reference in scope.
    ///
    /// # Examples
    ///
    /// ```
    ///
    ///
    /// let value = interpreter.inspect_variable("x", |var| {
    ///     if var.some_complicated_property  {
    ///         Value::Number(0.0)
    ///     } else {
    ///         Value::Number(1.0)
    ///     }
    /// });
    ///
    ///
    ///
    /// ```
    pub fn inspect_variable<F>(&mut self, name: &str, inspect: F) -> Result<Value, Exception>
    where
        F: FnOnce(&Value) -> Result<Value, Exception>,
    {
        // Look up in normal scope stack
        match self.resolve_variable(name) {
            // If not found, check if it is a property of the global object
            None => match self.global_object.borrow_mut().get(name) {
                None => Err(ReferenceError(name.to_owned())),
                Some(v) => inspect(v),
            },
            Some(v) => inspect(v),
        }
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> Result<Value, Exception> {
        self.edit_variable(name, |lvalue: &mut Value| -> Result<Value, Exception> {
            *lvalue = value.clone();
            Ok(value)
        })
    }

    pub fn notify_break(&mut self) {
        self.break_flag = true;
    }
    pub fn clear_break(&mut self) {
        self.break_flag = false;
    }
    pub fn broke(&self) -> bool {
        self.break_flag
    }

    pub fn notify_return(&mut self) {
        self.return_flag = true;
    }
    pub fn clear_return(&mut self) {
        self.return_flag = false;
    }
    pub fn returned(&self) -> bool {
        self.return_flag
    }

    pub fn suppress_declarations(&mut self) {
        self.declaration_suppression_counter += 1
    }
    pub fn allow_declarations(&mut self) {
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

    fn resolve_variable(&self, name: &str) -> Option<&Value> {
        self.scope_stack
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
    }

    fn populate_built_ins(global_object: Rc<RefCell<Box<dyn Object>>>) {
        global_object
            .borrow_mut()
            .put("console", Console::new().value());
    }

    fn handle_exception(&mut self, _exception: Exception) {
        #[cfg(not(feature = "suppress_exceptions"))]
        eprintln!("{}", _exception);
    }
}
