use resast::prelude::*;

use crate::{
    ir::{expression::*, marker::*, statement::*},
    runtime::*,
};


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
            Expr::Binary(b) => {
                BinaryExpression::boxed(
                    b.operator.into(),
                    (*b.left).into(),
                    (*b.right).into(),
                )
            }
            _ => unimplemented!(),
        }
    }
}

impl From<resast::expr::Expr<'_>> for Box<dyn Statement> {
    fn from(e: Expr<'_>) -> Self {
        match e {
            Expr::Ident(i) => ExpressionStatement::boxed(i.into()),
            Expr::Lit(l) => ExpressionStatement::boxed(l.into()),
            Expr::Binary(b) => ExpressionStatement::boxed(BinaryExpression::boxed(
                b.operator.into(),
                (*b.left).into(),
                (*b.right).into(),
            )),
            _ => unimplemented!(),
        }
    }
}

impl From<resast::decl::VarDecl<'_>> for Box<dyn Statement> {
    fn from(dec: VarDecl) -> Self {
        let VarDecl { id, mut init } = dec;
        if let Pat::Ident(id) = id {
            match init.take().unwrap() {
                Expr::Lit(l) => match l {
                    Lit::Number(n) => {
                        let n = n.parse::<f64>().unwrap();
                        VariableDeclaration::boxed(&id.name, Literal::boxed(Value::Number(n)))
                    }
                    Lit::Boolean(b) => {
                        VariableDeclaration::boxed(&id.name, Literal::boxed(Value::Boolean(b)))
                    }
                    _ => unimplemented!(),
                },
                Expr::Binary(bin_exp) => {
                    let binary_expression = BinaryExpression::boxed(
                        bin_exp.operator.into(),
                        (*bin_exp.left).into(),
                        (*bin_exp.right).into(),
                    );
                    VariableDeclaration::boxed(&id.name, binary_expression)
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
                super::parser::parse_block(b.0, &mut consequent_block);
                consequent_expr = Box::new(consequent_block);
            }
            _ => panic!("Unsupported if statement consequent type"),
        }

        if is.alternate.is_some() {
            match *is.alternate.unwrap() {
                Stmt::Expr(e) => alternate_expr = Some(e.into()),
                Stmt::Block(b) => {
                    super::parser::parse_block(b.0, &mut alternate_block);
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
        super::parser::parse_block(f.body.0, &mut block);
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
