use crate::{
    ir::{marker::Statement, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Eq, PartialEq)]
pub enum ScopeType {
    Function,
    Control,
    Global,
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct Scope {
    pub children: Vec<Box<dyn Statement>>,
    scope_type: ScopeType,
}

impl Scope {
    pub fn new(scope_type: ScopeType) -> Self {
        Self {
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

impl IrNode for Scope {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.scope_type);
        for child in self.children.iter() {
            output += &child.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.run(self.clone())
    }
}
