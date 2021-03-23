use resast::prelude::*;
use ressa::Parser;

use crate::ir::statement::*;

pub fn parse_block(statements: Vec<ProgramPart>, block: &mut Scope) {
    for part in statements {
        match part {
            ProgramPart::Decl(d) => match d {
                Decl::Var(_, mut dec) => {
                    for sub_dec in dec.drain(..) {
                        block.append(sub_dec.into());
                    }
                }
                Decl::Func(_) => panic!("Nested functions not supported"),
                _ => unimplemented!(),
            },
            ProgramPart::Stmt(s) => match s {
                Stmt::Return(e) => {
                    match e {
                        None => block.append(ReturnStatement::boxed_empty()),
                        Some(e) => block.append(ReturnStatement::boxed(e.into())),
                    }
                }
                Stmt::Var(mut v) => block.append(v.first_mut().unwrap().clone().into()),
                Stmt::If(is) => block.append(is.into()),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

pub fn parse_program(input: &str) -> Scope {
    // parse
    let mut parser = Parser::new(input).unwrap();
    let ast = parser.parse().expect("Failed to parse");

    // programmatically construct IR from AST
    let mut ir = Scope::named("Script");

    if let Program::Script(ast_nodes) = ast {
        for node in ast_nodes {
            match node {
                ProgramPart::Decl(dec) => match dec {
                    Decl::Var(VarKind::Let, mut dec) => {
                        for sub_dec in dec.drain(..) {
                            ir.append(sub_dec.into());
                        }
                    }
                    Decl::Func(f) => {
                        ir.append(f.into());
                    }
                    _ => panic!("Unsupported Declaration"),
                },
                ProgramPart::Stmt(s) => match s {
                    Stmt::Expr(e) => match e {
                        Expr::Call(c) => {
                            ir.append(c.into());
                        }
                        Expr::Ident(i) => {
                            ir.append(i.into());
                        }
                        _ => panic!("Unsupported expression"),
                    },
                    _ => panic!("Unsupported statement"),
                },
                ProgramPart::Dir(_) => panic!("Directives not supported"),
            }
        }
    }
    ir
}