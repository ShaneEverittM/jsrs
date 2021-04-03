use std::{collections::HashMap, fmt};

use crate::{
    ir::statement::{Scope, ScopeType},
    runtime::{Object, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Object, Clone)]
#[object_type(Function)]
pub struct Function {
    // TODO: Properties should contain the arguments, caller, callee
    //       and number of args while the function is executing as per
    //       https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions#the_arguments_object
    properties: HashMap<String, Value>,
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Scope,
    is_built_in: bool,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Scope) -> Box<Self> {
        Box::new(Self {
            properties: HashMap::new(),
            name,
            parameters,
            body,
            is_built_in: false,
        })
    }

    pub fn built_in(name: String, parameters: Vec<String>) -> Box<Self> {
        Box::new(Self {
            properties: HashMap::new(),
            name,
            parameters,
            body: Scope::new(ScopeType::Function),
            is_built_in: true,
        })
    }

    pub fn is_built_in(&self) -> bool {
        self.is_built_in
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function Object: {}", self.name)?;
        f.write_str(&self.format_properties())
    }
}
