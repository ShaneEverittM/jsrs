use crate::{
    ir::{
        marker::{BlockStatement, Statement},
        IRNode,
    },
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ScopeType {
    Function,
    Local,
    Global,
}

#[derive(Debug, Clone)]
pub struct Scope {
    name: String,
    pub children: Vec<Box<dyn Statement>>,
    scope_type: ScopeType,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            name: String::from("Block"),
            children: Vec::new(),
            scope_type: ScopeType::Local,
        }
    }
}

impl Scope {
    pub fn named(name: &str, scope_type: ScopeType) -> Self {
        Self {
            name: name.to_owned(),
            children: Vec::new(),
            scope_type,
        }
    }

    pub fn new(scope_type: ScopeType) -> Self {
        Self {
            name: "Scope".to_owned(),
            children: Vec::new(),
            scope_type,
        }
    }

    pub fn append(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
    }

    pub fn append_all(&mut self, statements: Vec<Box<dyn Statement>>) {
        for statement in statements {
            self.children.push(statement);
        }
    }

    pub fn get_type(&self) -> &ScopeType {
        &self.scope_type
    }
}

impl IRNode for Scope {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.name);
        for child in self.children.iter() {
            output += &child.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Value {
        interpreter.run(self.clone())
    }
}

impl Statement for Scope {}

impl BlockStatement for Scope {}
