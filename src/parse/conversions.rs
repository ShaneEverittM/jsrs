use std::string::ToString;

use resast::prelude::*;

use crate::{
    ir::{expression::*, marker::*, ops::*, statement::*},
    parse::parser::*,
    runtime::*,
};

// Helps Rust figure out e.into() when e is in a Box..
impl From<Box<resast::expr::Expr<'_>>> for Box<dyn Expression> {
    fn from(expr: Box<Expr<'_>>) -> Self {
        (*expr).into()
    }
}

impl From<Box<resast::stmt::Stmt<'_>>> for Box<dyn Statement> {
    fn from(expr: Box<Stmt<'_>>) -> Self {
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
    fn from(mut c: CallExpr<'_>) -> Self {
        let callee = c.callee;
        let arguments = c.arguments.drain(..).map(|e| e.into()).collect();
        match *callee {
            Expr::Ident(i) => CallExpression::boxed(&i.name, arguments),
            Expr::Member(m) => {
                let id: String = match *m.object {
                    Expr::Ident(i) => i.name.into(),
                    _ => panic!("expected ident"),
                };
                let property: String = match *m.property {
                    Expr::Ident(i) => i.name.into(),
                    _ => panic!("expected ident"),
                };
                CallExpression::boxed_member(&id, &property, arguments)
            }
            _ => {
                dbg!(callee);
                unimplemented!()
            }
        }
    }
}

impl From<resast::expr::AssignExpr<'_>> for Box<dyn Expression> {
    fn from(assn_expr: AssignExpr<'_>) -> Self {
        match assn_expr.operator {
            AssignOp::Equal => (),
            _ => unimplemented!(),
        }

        // TODO: Support patterns
        let e = match assn_expr.left {
            AssignLeft::Pat(_) => unimplemented!(),
            AssignLeft::Expr(e) => e,
        };

        AssignmentExpression::boxed(e.into(), assn_expr.right.into())
    }
}

impl From<resast::expr::UpdateExpr<'_>> for Box<dyn Expression> {
    fn from(up_expr: UpdateExpr<'_>) -> Self {
        let id = match *up_expr.argument {
            Expr::Ident(i) => i,
            _ => unimplemented!(),
        };
        let op = match up_expr.operator {
            UpdateOp::Increment => UnaryOperator::Increment,
            UpdateOp::Decrement => UnaryOperator::Decrement,
        };
        UpdateExpression::boxed(Variable::new(&id.name), op, up_expr.prefix)
    }
}

impl From<resast::expr::MemberExpr<'_>> for Box<dyn Expression> {
    fn from(mem_expr: MemberExpr<'_>) -> Self {
        // FIXME: Currently only support plain ident member expressions of depth 1
        let object = match *mem_expr.object {
            Expr::Ident(i) => i.name.to_string(),
            _ => {
                dbg!(mem_expr.object);
                unimplemented!()
            }
        };

        let property = match *mem_expr.property {
            Expr::Ident(i) => i.name.to_string(),
            _ => {
                dbg!(mem_expr.property);
                unimplemented!()
            }
        };
        MemberExpression::boxed(object, property)
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
            Expr::Update(up_expr) => up_expr.into(),
            Expr::Member(mem_expr) => mem_expr.into(),
            _ => unimplemented!(),
        }
    }
}

/* # Expression to Statement# */

impl From<resast::expr::Expr<'_>> for Box<dyn Statement> {
    fn from(expr: Expr<'_>) -> Self {
        ExpressionStatement::boxed(expr.into())
    }
}

/* # Statements # */

impl From<resast::stmt::IfStmt<'_>> for Box<dyn Statement> {
    fn from(is: IfStmt<'_>) -> Self {
        let test: Box<dyn Expression> = is.test.into();
        let consequent_expr: Box<dyn Statement> = is.consequent.into();

        let alternate_expr = is.alternate.map(|e| e.into());

        IfStatement::boxed(test, consequent_expr, alternate_expr)
    }
}

impl From<resast::stmt::ForStmt<'_>> for Box<dyn Statement> {
    fn from(for_stmt: ForStmt<'_>) -> Self {
        let test: Option<Box<dyn Expression>> = for_stmt.test.map(|t| t.into());
        let update: Option<Box<dyn Expression>> = for_stmt.update.map(|u| u.into());
        let body: Box<dyn Statement> = for_stmt.body.into();

        match for_stmt.init {
            None => ForStatement::boxed(None, None, test, update, body),
            Some(init) => match init {
                LoopInit::Variable(VarKind::Let, decls) => {
                    assert_eq!(decls.len(), 1);
                    let init_decl = decls.first().cloned().unwrap().into();
                    ForStatement::boxed(None, Some(init_decl), test, update, body)
                }
                LoopInit::Variable(_, _) => {
                    unimplemented!("Only let expressions supported in for loops")
                }
                LoopInit::Expr(e) => ForStatement::boxed(Some(e.into()), None, test, update, body),
            },
        }
    }
}

impl From<resast::stmt::BlockStmt<'_>> for Box<dyn Statement> {
    fn from(block_statement: BlockStmt<'_>) -> Self {
        let mut body_block = Scope::new(ScopeType::Control);
        parse_block(block_statement.0, &mut body_block);
        Box::new(body_block)
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
            Stmt::For(for_stmt) => for_stmt.into(),
            Stmt::Break(break_stmt) => match break_stmt {
                None => BreakStatement::boxed(),
                Some(_) => {
                    unimplemented!("Labeled break statements not supported")
                }
            },
            Stmt::Block(block_stmt) => block_stmt.into(),
            _ => {
                dbg!(stmt);
                unimplemented!()
            }
        }
    }
}

/* # Declarations # */

impl From<resast::decl::VarDecl<'_>> for Box<dyn Statement> {
    fn from(dec: VarDecl) -> Self {
        let VarDecl { id, mut init } = dec;
        if let Pat::Ident(id) = id {
            VariableDeclaration::boxed(
                &id.name,
                init.take()
                    .map(|e| e.into())
                    .unwrap_or_else(|| Literal::boxed(Value::Undefined) as Box<dyn Expression>),
            )
        } else {
            unimplemented!()
        }
    }
}

impl From<resast::Func<'_>> for Box<dyn Statement> {
    fn from(f: Func<'_>) -> Self {
        let mut block = Scope::new(ScopeType::Function);
        super::parser::parse_block(f.body.0, &mut block);
        let params = f
            .params
            .iter()
            .map(|param| match param {
                FuncArg::Expr(Expr::Ident(id)) => id.name.to_string(),
                FuncArg::Pat(Pat::Ident(id)) => id.name.to_string(),
                _ => panic!("Unsupported parameter ident"),
            })
            .collect();
        FunctionDeclaration::boxed(&f.id.unwrap().name, params, block)
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
