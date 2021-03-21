use crate::{
    ast::{marker::Statement, ASTNode},
    runtime::{Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct BlockStatement {
    name: String,
    pub children: Vec<Box<dyn Statement>>,
    // variables
    // function declarations
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self {
            name: String::from("Block"),
            children: Vec::new(),
        }
    }
}

impl BlockStatement {
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
impl ASTNode for BlockStatement {
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

impl Statement for BlockStatement {}
