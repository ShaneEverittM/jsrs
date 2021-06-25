use crate::{
    ir::{marker::Statement, IrNode},
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Eq, PartialEq)]
pub enum BlockType {
    Function,
    Control,
    Global,
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct Block {
    pub children: Vec<Box<dyn Statement>>,
    scope_type: BlockType,
}

impl Block {
    pub fn new(scope_type: BlockType) -> Self {
        Self {
            children: Vec::new(),
            scope_type,
        }
    }

    pub fn push(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
    }

    pub fn append(&mut self, mut statements: Vec<Box<dyn Statement>>) {
        self.children.append(&mut statements);
    }

    pub fn get_type(&self) -> BlockType {
        self.scope_type.clone()
    }
}

impl IrNode for Block {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.scope_type);
        for child in self.children.iter() {
            output += &child.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        interpreter.evaluate_scope(self.clone())
    }
}
