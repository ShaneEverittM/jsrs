use crate::{
    ir::{
        IRNode,
        marker::{BlockStatement, Statement},
    },
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ScopeType {
    Function,
    Control,
    Global,
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match self {
            ScopeType::Function => { "FunctionBody" }
            ScopeType::Control => { "ControlStatementBody" }
            ScopeType::Global => { "GlobalScope" }
        };

        f.write_str(id)
    }
}

#[derive(Debug, Clone)]
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

impl IRNode for Scope {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.scope_type);
        for child in self.children.iter() {
            output += &child.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Option<Value> {
        interpreter.run(self.clone())
    }
}

impl Statement for Scope {}

impl BlockStatement for Scope {}
