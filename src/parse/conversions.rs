use resast::prelude::*;

use crate::{
    ir::{expression::*, marker::*, statement::*},
    runtime::*,
};

// Helps Rust figure out e.into() when e is in a Box..
impl From<Box<resast::expr::Expr<'_>>> for Box<dyn Expression> {
    fn from(expr: Box<Expr<'_>>) -> Self {
        (*expr).into()
    }
}

/* # Expressions # */

impl From<resast::Ident<'_>> for Box<dyn Expression> {
    fn from(i: Ident<'_>) -> Self {
        Variable::boxed(&i.name)
    }
}

impl From<resast::expr::Lit<'_>> for Box<dyn Expression> {
    fn from(lit: Lit<'_>) -> Self {
        let value = match lit {
            Lit::Number(number) => Value::Number(number.parse::<f64>().unwrap()),
            Lit::Boolean(boolean) => Value::Boolean(boolean),
            Lit::String(string) => Value::String(string.clone_inner().into()),
            _ => unimplemented!(),
        };
        Literal::boxed(value)
    }
}

impl From<resast::expr::BinaryExpr<'_>> for Box<dyn Expression> {
    fn from(b: BinaryExpr<'_>) -> Self {
        BinaryExpression::boxed(b.operator.into(), b.left.into(), b.right.into())
    }
}

impl From<resast::expr::CallExpr<'_>> for Box<dyn Expression> {
    fn from(c: CallExpr<'_>) -> Self {
        let callee = c.callee;
        match *callee {
            Expr::Ident(i) => CallExpression::boxed(&i.name),
            _ => unimplemented!(),
        }
    }
}

impl From<resast::expr::AssignExpr<'_>> for Box<dyn Expression> {
    fn from(assn_expr: AssignExpr<'_>) -> Self {
        match assn_expr.operator {
            AssignOp::Equal => (),
            _ => unimplemented!(),
        }

        let id = match assn_expr.left {
            AssignLeft::Pat(p) => match p {
                Pat::Ident(i) => i,
                _ => unimplemented!(),
            },
            AssignLeft::Expr(e) => match *e {
                Expr::Ident(i) => i,
                _ => unimplemented!(),
            },
        };

        AssignmentExpression::boxed(Variable::new(&id.name), assn_expr.right.into())
    }
}

impl From<resast::expr::Expr<'_>> for Box<dyn Expression> {
    fn from(expr: Expr<'_>) -> Self {
        match expr {
            Expr::Ident(ident) => ident.into(),
            Expr::Lit(lit) => lit.into(),
            Expr::Binary(bin_expr) => bin_expr.into(),
            Expr::Call(call_expr) => call_expr.into(),
            Expr::Assign(assn_expr) => assn_expr.into(),
            _ => unimplemented!(),
        }
    }
}

/* # Statements # */

impl From<resast::expr::Expr<'_>> for Box<dyn Statement> {
    fn from(expr: Expr<'_>) -> Self {
        ExpressionStatement::boxed(expr.into())
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
            Stmt::Return(e) => match e {
                None => consequent_expr = ReturnStatement::boxed_empty(),
                Some(e) => consequent_expr = ReturnStatement::boxed(e.into()),
            },
            _ => panic!("Unsupported if statement consequent type"),
        }

        if is.alternate.is_some() {
            match *is.alternate.unwrap() {
                Stmt::Expr(e) => alternate_expr = Some(e.into()),
                Stmt::Block(b) => {
                    super::parser::parse_block(b.0, &mut alternate_block);
                    alternate_expr = Some(Box::new(alternate_block))
                }
                Stmt::Return(e) => match e {
                    None => alternate_expr = Some(ReturnStatement::boxed_empty()),
                    Some(e) => alternate_expr = Some(ReturnStatement::boxed(e.into())),
                },
                _ => panic!("Unsupported if statement consequent type"),
            }
        }

        IfStatement::boxed(test, consequent_expr, alternate_expr)
    }
}

impl From<resast::stmt::Stmt<'_>> for Box<dyn Statement> {
    fn from(stmt: Stmt<'_>) -> Self {
        match stmt {
            Stmt::Expr(expr) => expr.into(),
            Stmt::Return(ret_stmt) => match ret_stmt {
                None => ReturnStatement::boxed_empty(),
                Some(e) => ReturnStatement::boxed(e.into()),
            },
            Stmt::If(if_stmt) => if_stmt.into(),
            _ => unimplemented!(),
        }
    }
}

/* # Declarations # */

impl From<resast::decl::VarDecl<'_>> for Box<dyn Statement> {
    fn from(dec: VarDecl) -> Self {
        let VarDecl { id, mut init } = dec;
        if let Pat::Ident(id) = id {
            VariableDeclaration::boxed(&id.name, init.take().unwrap().into())
        } else {
            unimplemented!()
        }
    }
}

impl From<resast::Func<'_>> for Box<dyn Statement> {
    fn from(f: Func<'_>) -> Self {
        let mut block = Scope::default();
        super::parser::parse_block(f.body.0, &mut block);
        FunctionDeclaration::boxed(&f.id.unwrap().name, block)
    }
}

impl From<resast::decl::Decl<'_>> for Box<dyn Statement> {
    fn from(dec: Decl<'_>) -> Self {
        match dec {
            Decl::Func(f) => f.into(),
            _ => unimplemented!(),
        }
    }
}
