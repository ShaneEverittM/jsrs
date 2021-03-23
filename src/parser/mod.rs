use resast::prelude::*;
use ressa::Parser;

use crate::ir::statement::*;

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
