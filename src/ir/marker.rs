use std::fmt::Debug;

use resast::prelude::*;

use crate::{
    ir::{expression::*, statement::*, IRNode},
    runtime::*,
};

pub trait Statement: IRNode + Debug + StatementClone {}
pub trait Expression: IRNode + Debug + ExpressionClone {}

pub trait BlockStatement: IRNode + Debug + Statement {}
pub trait Declaration: Statement {}

pub trait StatementClone {
    fn clone_box(&self) -> Box<dyn Statement>;
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}

impl<T: 'static + Statement + Clone> StatementClone for T {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl<T: 'static + Expression + Clone> ExpressionClone for T {
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl From<f64> for Box<dyn Expression> {
    fn from(num: f64) -> Self {
        crate::ir::expression::Literal::boxed(crate::runtime::Value::Number(num))
    }
}

impl From<resast::expr::Lit<'_>> for Box<dyn Expression> {
    fn from(l: Lit<'_>) -> Self {
        match l {
            Lit::Number(n) => Literal::boxed(Value::Number(n.parse::<f64>().unwrap())),
            Lit::Boolean(b) => Literal::boxed(Value::Boolean(b)),
            _ => unimplemented!(),
        }
    }
}

impl From<resast::expr::Expr<'_>> for Box<dyn Expression> {
    fn from(e: Expr<'_>) -> Self {
        match e {
            Expr::Ident(i) => i.into(),
            Expr::Lit(l) => l.into(),
            Expr::Binary(b) => b.into(),
            _ => unimplemented!(),
        }
    }
}

impl From<resast::expr::Expr<'_>> for Box<dyn Statement> {
    fn from(e: Expr<'_>) -> Self {
        match e {
            Expr::Ident(i) => ExpressionStatement::boxed(i.into()),
            Expr::Lit(l) => ExpressionStatement::boxed(l.into()),
            _ => unimplemented!(),
        }
    }
}

impl From<resast::expr::BinaryExpr<'_>> for Box<dyn Expression> {
    fn from(bin_exp: BinaryExpr) -> Self {
        BinaryExpression::boxed(
            bin_exp.operator.into(),
            (*bin_exp.left).into(),
            (*bin_exp.right).into(),
        )
    }
}

impl From<resast::decl::VarDecl<'_>> for Box<dyn Statement> {
    fn from(dec: VarDecl) -> Self {
        let VarDecl { id, init } = dec;
        if let Pat::Ident(id) = id {
            match init.as_ref().unwrap() {
                Expr::Lit(l) => match l {
                    Lit::Number(n) => {
                        let n = n.parse::<f64>().unwrap();
                        VariableDeclaration::boxed(&id.name, Literal::boxed(Value::Number(n)))
                    }
                    Lit::Boolean(b) => {
                        VariableDeclaration::boxed(&id.name, Literal::boxed(Value::Boolean(*b)))
                    }
                    _ => unimplemented!(),
                },
                Expr::Binary(bin_exp) => {
                    VariableDeclaration::boxed(&id.name, bin_exp.clone().into())
                }
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    }
}

impl From<resast::expr::CallExpr<'_>> for Box<dyn Statement> {
    fn from(c: CallExpr<'_>) -> Self {
        let callee = c.callee;
        match *callee {
            Expr::Ident(i) => ExpressionStatement::boxed(CallExpression::boxed(&i.name)),
            _ => unimplemented!(),
        }
    }
}

fn parse_block(statements: Vec<ProgramPart>, block: &mut Scope) {
    for part in statements {
        match part {
            ProgramPart::Decl(d) => match d {
                Decl::Var(_, mut dec) => block.append(dec.first_mut().unwrap().clone().into()),
                Decl::Func(_) => panic!("Nested functions not supported"),
                _ => unimplemented!(),
            },
            ProgramPart::Stmt(s) => match s {
                Stmt::Return(e) => match e.unwrap() {
                    Expr::Binary(bin_exp) => block.append(ReturnStatement::boxed(bin_exp.into())),
                    Expr::Ident(id) => block.append(ReturnStatement::boxed(id.into())),
                    Expr::Lit(lit) => block.append(ReturnStatement::boxed(lit.into())),
                    _ => unimplemented!(),
                },
                Stmt::Var(mut v) => block.append(v.first_mut().unwrap().clone().into()),
                Stmt::If(is) => block.append(is.into()),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

impl From<resast::stmt::IfStmt<'_>> for Box<dyn Statement> {
    fn from(is: IfStmt<'_>) -> Self {
        let test: Box<dyn Expression> = is.test.into();
        let mut consequent_block = Scope::default();
        let consequent_expr: Box<dyn Statement>;
        let mut alternate_block = Scope::default();
        let mut alternate_expr: Option<Box<dyn Statement>> = None;

        match *is.consequent {
            Stmt::Expr(e) => consequent_expr = e.into(),
            Stmt::Block(b) => {
                parse_block(b.0, &mut consequent_block);
                consequent_expr = Box::new(consequent_block);
            }
            _ => panic!("Unsupported if statement consequent type"),
        }

        if is.alternate.is_some() {
            match *is.alternate.unwrap() {
                Stmt::Expr(e) => alternate_expr = Some(e.into()),
                Stmt::Block(b) => {
                    parse_block(b.0, &mut alternate_block);
                    alternate_expr = Some(Box::new(alternate_block))
                }
                _ => panic!("Unsupported if statement consequent type"),
            }
        }

        IfStatement::boxed(test, consequent_expr, alternate_expr)
    }
}

impl From<resast::Func<'_>> for Box<dyn Statement> {
    fn from(f: Func<'_>) -> Self {
        let mut block = Scope::default();
        parse_block(f.body.0, &mut block);
        FunctionDeclaration::boxed(&f.id.unwrap().name, block)
    }
}

impl From<resast::Ident<'_>> for Box<dyn Statement> {
    fn from(i: Ident<'_>) -> Self {
        ExpressionStatement::boxed(Variable::boxed(&i.name))
    }
}

impl From<resast::Ident<'_>> for Box<dyn Expression> {
    fn from(i: Ident<'_>) -> Self {
        Variable::boxed(&i.name)
    }
}
