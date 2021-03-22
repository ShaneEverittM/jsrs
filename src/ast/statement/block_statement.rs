use crate::{
    ast::{marker::{Statement, BlockStatement}, ASTNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct Scope {
    name: String,
    pub children: Vec<Box<dyn Statement>>,
    // variables
    // function declarations
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            name: String::from("Block"),
            children: Vec::new(),
        }
    }
}

impl Scope {
    pub fn named(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            children: Vec::new(),
        }
    }
    pub fn append(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
    }
}
impl ASTNode for Scope {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}{}\n", indent_str, self.name);
        for child in self.children.iter() {
            output += &child.dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, _interpreter: &mut Interpreter) -> Value {
        unimplemented!()
    }
}

impl Statement for Scope {}
impl BlockStatement for Scope {}
