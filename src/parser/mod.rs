#![allow(clippy::collapsible_match)]
use resast::prelude::{Program as ParsedProgram, *};
use ressa::Parser;

use crate::ast::expression::{BinaryExpression, CallExpression, Literal, Variable};
use crate::ast::{ops, statement::*};
use crate::runtime::Value;

pub fn parse_var_declaration(dec: &mut VarDecl) -> Box<VariableDeclaration> {
    let VarDecl { id, init } = dec;
    if let Pat::Ident(id) = id {
        match init.take().unwrap() {
            Expr::Lit(l) => match l {
                Lit::Number(n) => {
                    let n = n.parse::<f64>().unwrap();
                    VariableDeclaration::boxed(&id.name, Literal::boxed(Value::Number(n)))
                }
                _ => unimplemented!(),
            },
            Expr::Binary(bin_exp) => VariableDeclaration::boxed(&id.name, parse_bin_expr(bin_exp)),
            _ => unimplemented!(),
        }
    } else {
        unimplemented!()
    }
}

pub fn parse_bin_expr(bin_exp: BinaryExpr) -> Box<BinaryExpression> {
    match (*bin_exp.left, *bin_exp.right) {
        (Expr::Lit(le), Expr::Lit(re)) => match (le, re) {
            (Lit::Number(ln), Lit::Number(rn)) => {
                let ln = ln.parse::<f64>().unwrap();
                let rn = rn.parse::<f64>().unwrap();
                let op = match bin_exp.operator {
                    BinaryOp::Plus => ops::BinaryOp::Add,
                    BinaryOp::Minus => ops::BinaryOp::Subtract,
                    _ => unimplemented!(),
                };
                BinaryExpression::boxed(
                    op,
                    Literal::boxed(Value::Number(ln)),
                    Literal::boxed(Value::Number(rn)),
                )
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

pub fn parse_call_expr(c: CallExpr) -> Box<ExpressionStatement> {
    let callee = c.callee;
    match *callee {
        Expr::Ident(i) => ExpressionStatement::boxed(CallExpression::boxed(&i.name)),
        _ => unimplemented!(),
    }
}

pub fn parse_program(input: &str) -> Scope {
    // lex + parse
    let mut parser = Parser::new(input).expect("Failed to create parser");
    let script = parser.parse().expect("Failed to parse");
    // programmatically construct IR from parsed repr => program
    let mut program = Scope::named("Parsed");
    if let ParsedProgram::Script(parts) = script {
        for part in parts {
            match part {
                ProgramPart::Dir(_) => {
                    unimplemented!()
                }
                ProgramPart::Decl(dec) => match dec {
                    Decl::Var(_, mut dec) => {
                        program.append(parse_var_declaration(dec.first_mut().unwrap()))
                    }
                    Decl::Func(f) => {
                        let mut block = Scope::default();
                        for part in f.body.0 {
                            match part {
                                ProgramPart::Decl(d) => match d {
                                    Decl::Var(_, mut dec) => block
                                        .append(parse_var_declaration(dec.first_mut().unwrap())),
                                    _ => unimplemented!(),
                                },
                                ProgramPart::Stmt(s) => match s {
                                    Stmt::Return(e) => match e.unwrap() {
                                        Expr::Binary(bin_exp) => block.append(
                                            ReturnStatement::boxed(parse_bin_expr(bin_exp)),
                                        ),
                                        _ => unimplemented!(),
                                    },

                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            }
                        }
                        program.append(FunctionDeclaration::boxed(&f.id.unwrap().name, block));
                    }
                    Decl::Class(_) => {}
                    Decl::Import(_) => {}
                    Decl::Export(_) => {}
                },
                ProgramPart::Stmt(s) => match s {
                    Stmt::Expr(e) => match e {
                        Expr::Call(c) => {
                            program.append(parse_call_expr(c));
                        }
                        Expr::Ident(i) => {
                            program.append(ExpressionStatement::boxed(Variable::boxed(&i.name)));
                        }
                        _ => unimplemented!(),
                    },
                    _ => {
                        unimplemented!()
                    }
                },
            }
        }
    }
    program
}
