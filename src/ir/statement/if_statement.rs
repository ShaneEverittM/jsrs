use crate::{
    ir::{
        marker::{Expression, Statement},
        IrNode,
    },
    runtime::{exception::*, Interpreter, Value},
};

#[derive(Debug, Clone)]
pub struct IfStatement {
    test: Box<dyn Expression>,
    consequent: Box<dyn Statement>,
    alternate: Option<Box<dyn Statement>>,
}

impl IfStatement {
    pub fn new(
        test: Box<dyn Expression>,
        consequent: Box<dyn Statement>,
        alternate: Option<Box<dyn Statement>>,
    ) -> Self {
        Self {
            test,
            consequent,
            alternate,
        }
    }
    pub fn boxed(
        test: Box<dyn Expression>,
        consequent: Box<dyn Statement>,
        alternate: Option<Box<dyn Statement>>,
    ) -> Box<Self> {
        Box::new(Self {
            test,
            consequent,
            alternate,
        })
    }
}

impl IrNode for IfStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}IfStatement\n", indent_str);
        output += &self.test.dump(indent + 1);
        output += &self.consequent.dump(indent + 1);
        if self.alternate.is_some() {
            output += &self.alternate.as_ref().unwrap().dump(indent + 1);
        }
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        if let Ok(val) = self.test.evaluate(interpreter) {
            let test = match val {
                Value::Boolean(b) => b,
                _ => {
                    return Err(TypeError(
                        "If statement test must evaluate to boolean".to_string(),
                    ))
                }
            };

            if test {
                self.consequent.evaluate(interpreter)?;
            } else if self.alternate.is_some() {
                self.alternate.as_mut().unwrap().evaluate(interpreter)?;
            }
        }
        Ok(Value::Undefined)
    }
}

impl Statement for IfStatement {}
