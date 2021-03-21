use super::{ASTNode, Statement, Value};
use crate::runtime::Interpreter;

#[derive(Debug, Clone)]
pub struct Block {
    name: String,
    pub children: Vec<Box<dyn Statement>>,
    // variables
    // function declarations
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: String::from("Block"),
            children: Vec::new(),
        }
    }
}

impl Block {
    pub fn new(name: &str) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            children: Vec::new(),
        })
    }
    pub fn append(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
    }
}
impl ASTNode for Block {
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

impl Statement for Block {}
