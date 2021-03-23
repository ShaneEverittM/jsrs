#![allow(clippy::collapsible_match)]
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
impl From<resast::expr::BinaryExpr<'_>> for Box<dyn Expression> {
    fn from(bin_exp: BinaryExpr) -> Self {
        match (*bin_exp.left, *bin_exp.right) {
            (Expr::Lit(le), Expr::Lit(re)) => match (le, re) {
                (Lit::Number(ln), Lit::Number(rn)) => {
                    let ln = ln.parse::<f64>().unwrap();
                    let rn = rn.parse::<f64>().unwrap();
                    crate::ir::expression::BinaryExpression::boxed(
                        bin_exp.operator.into(),
                        ln.into(),
                        rn.into(),
                    )
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
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
                    _ => unimplemented!(),
                },
                Expr::Binary(bin_exp) => {
                    VariableDeclaration::boxed(&id.name, bin_exp.clone().into())
                },
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

impl From<resast::Func<'_>> for Box<dyn Statement> {
    fn from(f: Func<'_>) -> Self {
        let mut block = Scope::default();
        for part in f.body.0 {
            match part {
                ProgramPart::Decl(d) => match d {
                    Decl::Var(_, mut dec) => {
                        block.append(dec.first_mut().unwrap().clone().into())
                    }
                    _ => unimplemented!(),
                },
                ProgramPart::Stmt(s) => match s {
                    Stmt::Return(e) => match e.unwrap() {
                        Expr::Binary(bin_exp) => {
                            block.append(ReturnStatement::boxed(bin_exp.into()))
                        }
                        _ => unimplemented!(),
                    },

                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
        FunctionDeclaration::boxed(&f.id.unwrap().name, block)
    }
}

impl From<resast::Ident<'_>> for Box<dyn Statement> {
    fn from(i: Ident<'_>) -> Self {
        ExpressionStatement::boxed(Variable::boxed(&i.name))
    }
}
