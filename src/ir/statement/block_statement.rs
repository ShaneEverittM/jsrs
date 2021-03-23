use crate::{
    ir::{
        marker::{BlockStatement, Statement},
        statement::VariableDeclaration,
        IRNode,
    },
    runtime::{Interpreter, Value},
};


#[derive(Debug, Clone)]
pub struct Scope {
    name: String,
    pub children: Vec<Box<dyn Statement>>,
    pub variables: Vec<VariableDeclaration>, // function declarations
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            name: String::from("Block"),
            children: Vec::new(),
            variables: Vec::new(),
        }
    }
}

impl Scope {
    pub fn named(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            children: Vec::new(),
            variables: Vec::new(),
        }
    }
    pub fn append(&mut self, statement: Box<dyn Statement>) {
        self.children.push(statement);
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
