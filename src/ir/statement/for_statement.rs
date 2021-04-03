use crate::{
    ir::{
        marker::{Expression, Statement},
        IrNode,
    },
    runtime::{exception::*, Interpreter, Value},
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Statement, Clone)]
pub struct ForStatement {
    // ?: can a union be used here?
    initializer_expr: Option<Box<dyn Expression>>,
    initializer_decl: Option<Box<dyn Statement>>,
    test: Option<Box<dyn Expression>>,
    update: Option<Box<dyn Expression>>,
    body: Box<dyn Statement>,
}

impl ForStatement {
    pub fn boxed(
        initializer_expr: Option<Box<dyn Expression>>,
        initializer_decl: Option<Box<dyn Statement>>,
        test: Option<Box<dyn Expression>>,
        update: Option<Box<dyn Expression>>,
        body: Box<dyn Statement>,
    ) -> Box<Self> {
        assert!(
            (initializer_expr.is_none() || initializer_decl.is_none())
                && !(initializer_expr.is_none() && initializer_decl.is_none())
        );
        Box::new(Self {
            initializer_expr,
            initializer_decl,
            test,
            update,
            body,
        })
    }
}

impl IrNode for ForStatement {
    fn dump(&self, indent: u32) -> String {
        let indent_str = crate::util::make_indent(indent);
        let mut output = format!("{}ForStatement\n", indent_str);
        if self.initializer_decl.is_some() {
            output += &self.initializer_decl.as_ref().unwrap().dump(indent + 1);
        }
        if self.initializer_expr.is_some() {
            output += &self.initializer_expr.as_ref().unwrap().dump(indent + 1);
        }
        if self.test.is_some() {
            output += &self.test.as_ref().unwrap().dump(indent + 1);
        }
        if self.update.is_some() {
            output += &self.update.as_ref().unwrap().dump(indent + 1);
        }
        output += &self.body.dump(indent + 1);
        output
    }

    fn evaluate(&mut self, interpreter: &mut Interpreter) -> Result<Value, Exception> {
        if self.initializer_expr.is_some() {
            self.initializer_expr
                .as_mut()
                .unwrap()
                .evaluate(interpreter)?;
        }

        if self.initializer_decl.is_some() {
            self.initializer_decl
                .as_mut()
                .unwrap()
                .evaluate(interpreter)?;
        }

        if let Some(test) = self.test.as_mut() {
            while test.evaluate(interpreter) == Ok(Value::Boolean(true)) {
                self.body.evaluate(interpreter)?;

                if interpreter.broke() {
                    interpreter.clear_break();
                    break;
                }

                if interpreter.returned() {
                    break;
                }

                if self.update.is_some() {
                    self.update.as_mut().unwrap().evaluate(interpreter)?;
                }
            }
        } else {
            unimplemented!("For loops without tests not supported")
        }
        success!(Value::Undefined)
    }
}
